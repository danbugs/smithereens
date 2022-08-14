use anyhow::Result;

use crate::db_models::player::Player;
use crate::queries::player_getter::make_pidgtm_player_getter_query;
use diesel::{insert_into, prelude::*};
use schema::players::dsl::*;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::{env, process};
use std::{thread::sleep, time::Duration};

use crate::{db, schema};

pub async fn handle_map() -> Result<()> {
    tracing::info!(
        "❗ creating cache files at '{}'...",
        env::temp_dir().display()
    );
    let mut smithereens_emptyplayerid_cache_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(env::temp_dir().join(".smithereens_emptyplayerid_cache"))?;

    let mut smithereens_playerid_cache_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(env::temp_dir().join(".smithereens_playerid_cache"))?;

    tracing::info!("❗ checking cache for last checked player id...");
    let mut curr_player_id = if let Some(line) = BufReader::new(&smithereens_playerid_cache_file)
        .lines()
        .next()
    {
        line?.parse::<i32>()?
    } else {
        0
    };

    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            tracing::info!("❗ updating last checked player id cache, and exiting...");
            writeln!(smithereens_playerid_cache_file, "{}", curr_player_id)
                .expect("❌ failed to update smithereens playerid cache...");
        } else {
            process::exit(0);
        }
    })?;

    let db_connection = db::connect()?;
    let mut now = Instant::now();
    loop {
        tracing::info!("🤔 querying startgg api to get player based on curr_player id...");
        #[allow(unused_assignments)]
        let mut player_to_insert = None;
        // vvv logic to make requests until one of them suceeds
        loop {
            if let Ok(res) = make_pidgtm_player_getter_query(curr_player_id).await {
                player_to_insert = res;
                break;
            } else {
                let elapsed_time = Instant::now() - now;
                // vvv time until we're well within safe margins of the StartGG rate limit
                // 1 minute + 10% of a minute for safety
                let time_until_ok = Duration::from_secs(66) - elapsed_time;
                if time_until_ok.as_secs() > 0 {
                    tracing::info!(
                        "😴 sleeping for {:?} to ease off of the StartGG API's rate limit...",
                        time_until_ok
                    );
                    sleep(time_until_ok);
                    now = Instant::now();
                }
            }
        }

        if let Some(pti) = player_to_insert {
            if pti.player.user.is_none() {
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
            writeln!(smithereens_emptyplayerid_cache_file, "{}", curr_player_id)?;
        }

        curr_player_id += 1;

        if running.load(Ordering::SeqCst) > 0 {
            break;
        }
    }

    Ok(())
}
