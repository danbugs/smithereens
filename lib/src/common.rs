#![allow(unused)]

use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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

pub fn init_logger() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

use startgg::{GQLData, GQLVars};

#[allow(clippy::too_many_arguments)]
pub async fn start_read_all_by_increment_execute_finish_maybe_cancel<V, F, D>(
    is_cli: bool,
    gql_vars: Arc<Mutex<V>>,
    get_pages: fn(i32, Arc<Mutex<V>>) -> F,
    start: i32,
    end: Option<i32>,
    execute: fn(i32, D) -> Result<bool>,
    increment: fn(i32) -> Result<i32>,
    finish: fn(Arc<Mutex<V>>) -> Result<()>,
    cancel: fn(i32) -> Result<()>,
) -> Result<()>
where
    V: GQLVars + Clone,
    F: Future<Output = Result<D>>,
    D: GQLData,
{
    let running = Arc::new(AtomicUsize::new(0));

    if is_cli {
        let r = running.clone();
        ctrlc::set_handler(move || {
            let prev = r.fetch_add(1, Ordering::SeqCst);
            if prev == 0 {
                tracing::info!("👋 exiting...");
            } else {
                process::exit(0);
            }
        })?;
    }

    let mut curr_page = start;
    let mut now = Instant::now();
    'outer: loop {
        let result;
        'inner: loop {
            tracing::info!("🍥 querying StartGG API...");
            match get_pages(curr_page, gql_vars.clone()).await {
                Ok(data) => {
                    tracing::info!("🍥 got data for page {}", &curr_page);
                    result = data;
                    break 'inner;
                }
                Err(e) => {
                    if e.to_string().contains("429")
                        || e.to_string()
                            .contains("Our services aren't available right now")
                        || e.to_string().contains("error sending request for url")
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
                        tracing::error!(
                            "🙃 an oddity happened, skipping for now ({:#?})...",
                            e.to_string()
                        );

                        // weird error, skip this iteration
                        if e.to_string()
                            .contains("Look at json field for more details")
                        {
                            // StartGG doesn't allow querying more than 10,000th entry, so we stop here.
                            tracing::info!(
                                "🏁 got 'Look at json field for more details' at page {}!",
                                curr_page
                            );
                            break 'outer;
                        }

                        // else, if set getter, we prob. want to panic
                        let mut gql_vars_lock = gql_vars.lock().unwrap();
                        *gql_vars_lock = gql_vars_lock.update();
                    }
                }
            }
        }

        if execute(curr_page, result)? {
            break 'outer;
        } else {
            curr_page = increment(curr_page)?;
        }

        if Some(curr_page) == end {
            tracing::info!("🏁 reached the end criteria for this job!");
            break 'outer;
        }

        if is_cli && running.load(Ordering::SeqCst) > 0 {
            cancel(curr_page)?;
            break 'outer;
        }
    }

    finish(gql_vars)?;
    Ok(())
}
