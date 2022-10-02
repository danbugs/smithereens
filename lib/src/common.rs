#![allow(unused)]

use anyhow::Result;

use std::{
    future::Future,
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::{Duration, Instant},
};

use startgg::{GQLData, GQLVars};

pub async fn start_read_all_by_increment_execute_finish_maybe_cancel<V, F, D>(
    gql_vars: Arc<Mutex<V>>,
    get_pages: fn(i32, Arc<Mutex<V>>) -> F,
    start: fn() -> Result<i32>,
    execute: fn(Arc<Mutex<V>>, D) -> Result<bool>,
    increment: fn(i32) -> Result<i32>,
    finish: fn(Arc<Mutex<V>>) -> Result<()>,
    cancel: fn(Arc<Mutex<V>>) -> Result<()>,
) -> Result<()>
where
    V: GQLVars + Clone,
    F: Future<Output = Result<D>>,
    D: GQLData,
{
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

    let mut curr_page = start()?;
    let mut now = Instant::now();
    loop {
        let result;
        loop {
            tracing::info!("🍥 querying StartGG API...");
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

        if execute(gql_vars.clone(), result)? {
            break;
        } else {
            curr_page = increment(curr_page)?;
        }

        if running.load(Ordering::SeqCst) > 0 {
            cancel(gql_vars.clone())?;
            break;
        }
    }

    finish(gql_vars)?;
    Ok(())
}
