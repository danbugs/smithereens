#![allow(unused_assignments)]

use anyhow::Result;

use std::{
    future::Future,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use startgg::{GQLData, GQLVars};

pub async fn read_all_and_execute<V, F, D>(
    gql_vars: Arc<Mutex<V>>,
    get_pages: fn(i32, Arc<Mutex<V>>) -> F,
    execute: fn(D) -> Result<bool>,
    finish: fn(Arc<Mutex<V>>) -> Result<()>,
) -> Result<()>
where
    V: GQLVars + Clone,
    F: Future<Output = Result<D>>,
    D: GQLData,
{
    let mut curr_page = 1;

    let mut now = Instant::now();
    loop {
        let result;
        loop {
            tracing::info!("🍥 querying StartGG API for player results...");
            match get_pages(curr_page, gql_vars.clone()).await {
                Ok(data) => {
                    result = data;
                    break;
                }
                Err(e) => {
                    if e.to_string().contains("429")
                        || e.to_string()
                            .contains("Our services aren't available right now")
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
                        time_until_ok, &e.to_string()
                    );
                            sleep(time_until_ok);
                            now = Instant::now();
                        }
                    } else {
                        tracing::info!("🙃 an oddity happened, skipping for now...");
                        let mut gql_vars_lock = gql_vars.lock().unwrap();
                        *gql_vars_lock = gql_vars_lock.update();
                    }
                }
            }
        }

        if execute(result)? {
            break;
        } else {
            curr_page += 1;
        }
    }

    finish(gql_vars)?;
    Ok(())
}
