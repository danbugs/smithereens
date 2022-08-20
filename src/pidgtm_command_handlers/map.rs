use anyhow::Result;
use diesel::dsl::max;

use crate::db_models::empty_player_ids::EmptyPlayerId;
use crate::db_models::last_checked_player_id::LastCheckedPlayerId;
use crate::db_models::player::Player;
use crate::queries::player_getter::{make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData};
use crate::schema::last_checked_player_id;
use diesel::{insert_into, prelude::*};
use schema::empty_player_ids::dsl::*;
use schema::last_checked_player_id::dsl::*;
use schema::players::dsl::*;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::process;
use std::{thread::sleep, time::Duration};

use crate::{db, schema};

pub async fn handle_map() -> Result<()> {
    let db_connection = db::connect()?;

    tracing::info!("❗ checking cache for last checked player id...");
    let max_checked_player_id = last_checked_player_id
        .select(max(last_checked_player_id::player_id))
        .first::<Option<i32>>(&db_connection)?;
    let mut curr_player_id = if let Some(val) = max_checked_player_id {
        val
    } else {
        1
        // ^^^ don't start at 0 because it is uniquely populated
        // w/ all null values but non null player
    };

    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            tracing::info!("👋 exiting...");
        } else {
            process::exit(0);
        }
    })?;

    let mut now = Instant::now();
    loop {
        tracing::info!("🤔 querying startgg api to get player based on curr_player id...");

        // vvv compile doesn't know that the loop will ever stop running
        // so this is just for convenience
        #[allow(unused_assignments)]
        let mut player_to_insert = PIDGTM_PlayerGetterData::empty();

        // vvv logic to make requests until one of them suceeds
        loop {
            let req = make_pidgtm_player_getter_query(curr_player_id).await;
            if let Ok(res) = req {
                player_to_insert = res;
                break;
            } else {
                let err_msg = req.err().unwrap().to_string();
                // fine to unwrap, we know we've hit an error
                if err_msg.contains("429")
                    || err_msg.contains("Our services aren't available right now")
                {
                    // 429 (too many reqs) or outage, want to wait it out
                    let elapsed_time = Instant::now() - now;
                    let a_bit_over_1m = Duration::from_secs(66);
                    let time_until_ok = if elapsed_time < a_bit_over_1m {
                        a_bit_over_1m - elapsed_time
                    } else {
                        a_bit_over_1m
                    };
                    // ^^^ time until we're well within safe margins of the
                    // StartGG rate limit 1 minute + 10% of a minute for safety

                    if time_until_ok.as_secs() > 0 {
                        tracing::info!(
                        "😴 sleeping for {:?} to ease off of the StartGG API's rate limit ({:?})...",
                        time_until_ok, &err_msg
                    );
                        sleep(time_until_ok);
                        now = Instant::now();
                    }
                } else {
                    tracing::info!(
                        "🙃 an oddity happened on player id '{}', skipping for now...",
                        curr_player_id
                    );
                    curr_player_id += 1;
                    // some oddity happened, we can skip this player
                    // e.g., id exists, but all fields are null (231798)
                }
            }
        }

        if let Some(pti) = player_to_insert.player {
            if pti.user.is_none() || pti.user.as_ref().unwrap().slug.is_none() {
                tracing::info!(
                    "🧪 caught a test account (id: '{}'), skipping addition to pidgtm db...",
                    curr_player_id
                );
            } else {
                tracing::info!(
                    "💫 appending player (id: '{}') to pidgtm db...",
                    curr_player_id
                );
                insert_into(players)
                    .values(Player::from(pti))
                    .execute(&db_connection)?;
            }
        } else {
            tracing::info!("⛔ no player under id '{}', moving on...", curr_player_id);
            insert_into(empty_player_ids)
                .values(EmptyPlayerId::from(curr_player_id))
                .execute(&db_connection)?;
        }

        curr_player_id += 1;

        if running.load(Ordering::SeqCst) > 0 {
            tracing::info!("❗ updating smithereens player id cache file...");
            insert_into(last_checked_player_id)
                .values(LastCheckedPlayerId::from(curr_player_id))
                .execute(&db_connection)?;
            break;
        }
    }

    Ok(())
}
