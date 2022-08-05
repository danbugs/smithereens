use anyhow::Result;

use crate::db_models::player::Player;
use crate::queries::player_getter::make_pidgtm_player_getter_query;
use diesel::dsl::max;
use diesel::{insert_into, prelude::*};
use schema::players::dsl::*;

use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::{thread::sleep, time::Duration};

use crate::{db, schema};

// to be safe, I'll use 75% (i.e., 60 reqs / 60 secs)
// of the maximum allowed amount (i.e., 80 reqs / 60 secs)
// even then, when appending players onto the `pidgtm` DB,
// the public website should be put under maintenance.
const MAX_NUM_REQS_PER_60_SECS: i32 = 60;

pub async fn handle_map() -> Result<()> {
    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            tracing::info!("exiting...")
        } else {
            process::exit(0);
        }
    })?;

    tracing::info!("❗ querying pidgtm db to get the latest appended player id...");
    let db_connection = db::connect()?;
    let mut curr_player_id = if let Some(last_appended_player_id) =
        players
            .select(max(player_id))
            .first::<Option<i32>>(&db_connection)?
    {
        last_appended_player_id + 1
    } else {
        0
    };

    loop {
        let mut num_requests = 0;
        let now = Instant::now();
        while num_requests < MAX_NUM_REQS_PER_60_SECS {
            tracing::info!("🤔 querying startgg api to get player based on curr_player id...");
            let player_to_insert = make_pidgtm_player_getter_query(curr_player_id).await;
            num_requests += 1;

            if let Some(pti) = player_to_insert {
                tracing::info!(
                    "💫 appending player (id: '{}') to pidgtm db...",
                    curr_player_id
                );
                insert_into(players)
                    .values(Player::from(pti))
                    .execute(&db_connection)?;
            } else {
                tracing::info!("⛔ no player under id '{}', moving on...", curr_player_id);
            }

            curr_player_id += 1;
        }

        let elapsed_time = now.elapsed();
        let one_minute = Duration::from_secs(60);
        if elapsed_time >= one_minute {
            sleep(Duration::from_secs(20))
            // ^^^ sleeping for 20s just to be safe
        } else {
            // vvv time until we're well within safe margins of the StartGG rate limit (i.e., 80 reqs / 60 secs)
            let time_until_ok = one_minute - elapsed_time;
            if time_until_ok.as_secs() > 0 {
                tracing::info!(
                    "😴 sleeping for {:?} to avoid hitting the StartGG API's rate limit...",
                    time_until_ok
                );
                sleep(time_until_ok);
            }
        }

        if running.load(Ordering::SeqCst) > 0 {
            break;
        }
    }

    Ok(())
}
