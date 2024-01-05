#![allow(unused)]

use anyhow::Result;
use as_any::Downcast;
use startgg::{
    Entrant, Event, PhaseGroup, Player, Score, Seed, Set as SGGSet, SetSlot, Standing,
    StandingConnection, StandingStats, Tournament, User, Videogame,
};
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

use startgg::{queries::set_getter::SetGetterVars, GQLData, GQLVars};

use crate::{
    error_logs::insert_error_log, game::delete_games_from_requester_id,
    set::delete_sets_by_requester_id, tournament::delete_tournaments_from_requester_id,
};

pub fn init_logger() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

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

    if is_cli && std::env::var("CTRLC_HANDLER_SET").is_err() {
        let r = running.clone();
        ctrlc::set_handler(move || {
            let prev = r.fetch_add(1, Ordering::SeqCst);
            if prev == 0 {
                tracing::info!("👋 exiting...");
            } else {
                process::exit(0);
            }
        })?;

        // set env var indicating ctrlc handler is set
        std::env::set_var("CTRLC_HANDLER_SET", "true");
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
                        let err_msg = format!(
                            "🙃 an oddity happened, skipping for now ({:#?})...",
                            e.to_string()
                        );
                        tracing::error!(err_msg);
                        insert_error_log(err_msg.to_string())?;

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

                        // downcast, if setgettervars do x, else ...
                        let gql_vars_clone = gql_vars.clone();
                        let mut gql_vars_lock = gql_vars_clone.lock().unwrap();
                        if let Some(set_getter_vars) =
                            gql_vars_clone.downcast_ref::<SetGetterVars>()
                        {
                            let err_msg = format!("❌ something went wrong when aggregating data for player id: {}, deleting all of this player's games/sets/tourneys and skipping for now...", set_getter_vars.playerId);
                            tracing::error!(err_msg);
                            insert_error_log(err_msg.to_string())?;
                            // delete all player's games, sets, and tournaments
                            delete_games_from_requester_id(set_getter_vars.playerId)?;
                            delete_sets_by_requester_id(set_getter_vars.playerId)?;
                            delete_tournaments_from_requester_id(set_getter_vars.playerId)?;
                        } else {
                            let mut gql_vars_lock = gql_vars_clone.lock().unwrap();
                            *gql_vars_lock = gql_vars_lock.update();
                        }
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

pub fn get_sggset_test_data() -> SGGSet {
    SGGSet {
        id: 44887323,
        games: None,
        slots: vec![
            SetSlot {
                entrant: Some(Entrant {
                    id: Some(9412484),
                    name: Some("Ancient 3 scrub".to_string()),
                }),
                seed: Some(Seed {
                    seedNum: Some(2),
                    entrant: None,
                }),
                standing: Some(Standing {
                    entrant: None,
                    player: None,
                    stats: Some(StandingStats {
                        score: Score { value: Some(2.0) },
                    }),
                    placement: None,
                }),
            },
            SetSlot {
                entrant: Some(Entrant {
                    id: Some(9410060),
                    name: Some("tyrese".to_string()),
                }),
                seed: Some(Seed {
                    seedNum: Some(10),
                    entrant: None,
                }),
                standing: Some(Standing {
                    entrant: None,
                    player: None,
                    stats: Some(StandingStats {
                        score: Score { value: Some(0.0) },
                    }),
                    placement: None,
                }),
            },
        ],
        completedAt: Some(1645848034),
        phaseGroup: Some(PhaseGroup {
            bracketType: "DOUBLE_ELIMINATION".to_string(),
        }),
        event: Some(Event {
            id: Some(685122),
            slug: Some(
                "tournament/vsb-novice-friday-17/event/novice-ultimate-singles-bc-cooler-set"
                    .to_string(),
            ),
            phases: None,
            name: Some("Novice Ultimate Singles (BC Cooler Set 😎)".to_string()),
            numEntrants: Some(12),
            isOnline: Some(false),
            videogame: Some(Videogame {
                name: "Super Smash Bros. Ultimate".to_string(),
            }),
            tournament: Some(Tournament {
                id: 423456,
                name: "VSB - Novice Friday #17".to_string(),
                endAt: Some(1645862340),
            }),
            standings: Some(StandingConnection {
                nodes: vec![Standing {
                    entrant: Some(Entrant {
                        id: Some(9410060),
                        name: None,
                    }),
                    player: Some(Player {
                        id: 2021528,
                        prefix: None,
                        gamerTag: None,
                        user: Some(User {
                            name: None,
                            location: None,
                            bio: None,
                            birthday: None,
                            images: None,
                            slug: Some("user/1f1bee01".to_string()),
                            genderPronoun: None,
                            authorizations: None,
                        }),
                        rankings: None,
                        sets: None,
                    }),
                    stats: None,
                    placement: Some(5),
                }],
            }),
            teamRosterSize: None,
        }),
    }
}
