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

    tracing::info!("querying pidgtm db to get the latest appended player id...");
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
            tracing::info!("querying startgg api to get player based on curr_player id...");
            let player_to_insert = make_pidgtm_player_getter_query(curr_player_id).await?;

            tracing::info!("appending player to pidgtm db...");
            insert_into(players)
                .values(Player::from(player_to_insert))
                .execute(&db_connection)?;

            num_requests += 1;
            curr_player_id += 1;
        }

        // vvv time until we're well within safe margins of the StartGG rate limit (i.e., 80 reqs / 60 secs)
        let time_until_ok = Duration::from_secs(60) - now.elapsed();
        if time_until_ok.as_secs() > 0 {
            sleep(time_until_ok);
        }

        if running.load(Ordering::SeqCst) > 0 {
            break;
        }
    }

    Ok(())
}
