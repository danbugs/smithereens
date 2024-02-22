use anyhow::Result;
use as_any::Downcast;
use smithe_database::{
    db_models::{
        consecutive_group_result::ConsecutiveGroupResult, player::Player, set::Set,
        tournament::Tournament,
    },
    schema::{
        player_games::dsl::*, player_sets::dsl::*, player_tournaments::dsl::*, players::dsl::*,
    },
};

use diesel_async::{AsyncPgConnection, RunQueryDsl};

use diesel::{
    delete,
    dsl::{count, max},
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
    sql_query, update,
};
use smithe_database::{
    db_models::empty_player_ids::EmptyPlayerId, schema::empty_player_ids::dsl::*,
};
use startgg::{
    queries::set_getter::{SetGetterData, SetGetterVars},
    GQLData, GQLVars, Player as SGGPlayer, User,
};

use crate::{
    error_logs::insert_error_log,
    game::{delete_games_from_requester_id, maybe_get_games_from_set},
    set::{delete_sets_by_requester_id, get_opponent_set_slot, get_requester_set_slot},
    tournament::{
        delete_tournaments_from_requester_id, get_placement, get_requester_id_from_standings,
        get_seed, is_ssbu_singles_and_supported_tournament, is_tournament_cached,
        is_tournament_finished,
    },
};

pub async fn get_highest_id_with_sets_between(start_id: i32, end_id: i32) -> Result<Option<i32>> {
    let mut db_connection = smithe_database::connect().await?;

    let highest_id_player = smithe_database::schema::players::table
        .filter(smithe_database::schema::players::player_id.ge(start_id))
        .filter(smithe_database::schema::players::player_id.le(end_id))
        .inner_join(
            smithe_database::schema::player_sets::table
                .on(smithe_database::schema::players::player_id
                    .eq(smithe_database::schema::player_sets::requester_id)),
        )
        .select(smithe_database::schema::players::player_id)
        .order(smithe_database::schema::players::player_id.desc())
        .first::<i32>(&mut db_connection)
        .await
        .optional()?;

    Ok(highest_id_player)
}

pub async fn get_all_like(tag: &str) -> Result<Vec<Player>> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    let mut db_connection = smithe_database::connect().await?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag.ilike(format!("%{}%", processed_tag))) // case-insensitive like
        .get_results::<Player>(&mut db_connection)
        .await?;

    Ok(matching_players)
}

pub async fn get_player_from_slug(slug: &str) -> Result<Player> {
    let mut db_connection = smithe_database::connect().await?;
    let db_user_slug = format!("user/{}", slug);
    let matched_player = players
        .filter(smithe_database::schema::players::user_slug.eq(db_user_slug))
        .get_result::<Player>(&mut db_connection)
        .await?;

    Ok(matched_player)
}

pub async fn get_player(pid: i32) -> Result<Player> {
    let mut db_connection = smithe_database::connect().await?;
    let matched_player = players
        .filter(smithe_database::schema::players::player_id.eq(pid)) // case-insensitive like
        .get_result::<Player>(&mut db_connection)
        .await?;

    Ok(matched_player)
}

pub async fn add_new_player_to_pidgtm_db(pti: &SGGPlayer) -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;

    add_new_player_to_pidgtm_db_provided_connection(pti, &mut db_connection).await
}

async fn add_new_player_to_pidgtm_db_provided_connection(
    pti: &SGGPlayer,
    db_connection: &mut AsyncPgConnection,
) -> Result<()> {
    match insert_into(players)
        .values(Player::from(pti.clone()))
        .execute(db_connection)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                tracing::info!("👍 player already exists in pidgtm db, updating...");
                update_and_handle_deleted(pti).await
            }
            _ => Err(e.into()), // Propagate other errors
        },
    }
}

async fn update_player_in_pidgtm_db(pti: &SGGPlayer) -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;
    update_player_in_pidgtm_db_provided_connection(pti, &mut db_connection).await
}

async fn update_player_in_pidgtm_db_provided_connection(
    pti: &SGGPlayer,
    db_connection: &mut AsyncPgConnection,
) -> Result<()> {
    let player = Player::from(pti.clone());
    update(players)
        .filter(smithe_database::schema::players::player_id.eq(player.player_id))
        .set((
            prefix.eq(player.prefix),
            gamer_tag.eq(player.gamer_tag),
            name.eq(player.name),
            state.eq(player.state),
            country.eq(player.country),
            profile_picture.eq(player.profile_picture),
            twitch_username.eq(player.twitch_username),
            twitter_username.eq(player.twitter_username),
            gender_pronouns.eq(player.gender_pronouns),
            birthday.eq(player.birthday),
            bio.eq(player.bio),
            rankings.eq(player.rankings),
        ))
        .execute(db_connection)
        .await?;
    Ok(())
}

async fn update_and_handle_deleted(pti: &SGGPlayer) -> Result<()> {
    if pti.user.is_none() {
        tracing::info!("❎ caught a deleted account #1 (id: '{}')...", pti.id);
        pti.clone().user = get_empty_user_with_slug(pti.id).await?;
    } else if pti.user.as_ref().unwrap().slug.is_none() {
        tracing::info!("❎ caught a deleted account #2 (id: '{}')...", pti.id);
        pti.clone().user = match pti.clone().user.as_mut() {
            Some(u) => {
                // Perform the asynchronous operation and await it outside of the map.
                let future = async {
                    u.slug = get_empty_user_with_slug(pti.id)
                        .await
                        .unwrap()
                        .unwrap()
                        .slug;
                    u.clone()
                };
                // You now have a future that you can await.
                Some(future.await)
            }
            None => None,
        };
    } else {
        tracing::info!("💫 updating player (id: '{}')...", pti.id);
    }

    update_player_in_pidgtm_db(pti).await
}

pub async fn add_new_empty_player_record(pid: i32) -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;
    add_new_empty_player_record_provided_connection(pid, &mut db_connection).await?;
    Ok(())
}

async fn add_new_empty_player_record_provided_connection(
    pid: i32,
    db_connection: &mut AsyncPgConnection,
) -> Result<()> {
    insert_into(empty_player_ids)
        .values(EmptyPlayerId::from(pid))
        .on_conflict_do_nothing()
        .execute(db_connection)
        .await?;
    Ok(())
}

pub async fn get_subsequent_player_id_with_circle_back(some_id: i32) -> Result<i32> {
    let mut db_connection = smithe_database::connect().await?;
    let res = players
        .select(smithe_database::schema::players::player_id)
        .filter(smithe_database::schema::players::player_id.gt(some_id))
        .order(smithe_database::schema::players::player_id.asc())
        .first(&mut db_connection)
        .await
        .optional()?;

    if let Some(r) = res {
        Ok(r)
    } else {
        Ok(1000) // circle back logic
    }
}

pub async fn check_if_large_consecutive_playerid_grouping_exists() -> Result<bool> {
    let mut db_connection = smithe_database::connect().await?;
    let res: Vec<ConsecutiveGroupResult> = sql_query(
        r#"WITH RankedPlayerIDs AS (
            SELECT player_id, 
                   player_id - LAG(player_id) OVER (ORDER BY player_id) AS diff
            FROM empty_player_ids
        ),
        ConsecutiveGroups AS (
            SELECT player_id,
                   SUM(CASE WHEN diff > 1 THEN 1 ELSE 0 END) OVER (ORDER BY player_id) AS grp
            FROM RankedPlayerIDs
        )
        SELECT grp, COUNT(*) AS consecutive_count
        FROM ConsecutiveGroups
        GROUP BY grp
        HAVING COUNT(*) > 1144;"#,
    )
    .load(&mut db_connection)
    .await?; // 1144 is the number of players in the largest consecutive grouping

    // if vec is not empty, then there is a large consecutive grouping
    Ok(!res.is_empty())
}

pub async fn delete_large_consecutive_playerid_grouping() -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;
    let max_player_id = get_max_player_id().await?;

    // delete all empty_player_ids that are greater than the max player id
    delete(empty_player_ids)
        .filter(smithe_database::schema::empty_player_ids::player_id.gt(max_player_id))
        .execute(&mut db_connection)
        .await?;

    Ok(())
}

pub async fn get_max_player_id() -> Result<i32> {
    let mut db_connection = smithe_database::connect().await?;
    let max_player_id = players
        .select(max(smithe_database::schema::players::player_id))
        .first::<Option<i32>>(&mut db_connection)
        .await?;
    if let Some(val) = max_player_id {
        Ok(val)
    } else {
        Ok(1000) // nothing in db, start at 1000
    }
}

pub async fn get_empty_user_with_slug(pid: i32) -> Result<Option<User>> {
    let mut db_connection = smithe_database::connect().await?;
    get_empty_user_with_slug_provided_connection(pid, &mut db_connection).await
}

async fn get_empty_user_with_slug_provided_connection(
    pid: i32,
    db_connection: &mut AsyncPgConnection,
) -> Result<Option<User>> {
    let some_slug = players
        .select(user_slug)
        .filter(smithe_database::schema::players::player_id.eq(pid))
        .get_result(db_connection)
        .await
        .optional()?;

    Ok(Some(User {
        name: None,
        location: None,
        bio: None,
        birthday: None,
        images: None,
        genderPronoun: None,
        authorizations: None,
        slug: some_slug,
    }))
}

pub async fn maybe_delete_player_records<V>(maybe_sgv: V) -> Result<bool>
where
    V: GQLVars + Clone + 'static,
{
    if let Some(set_getter_vars) = maybe_sgv.downcast_ref::<SetGetterVars>() {
        let err_msg = format!("❌ something went wrong when aggregating data for player id: {}, deleting all of this player's games/sets/tourneys and skipping for now...", set_getter_vars.playerId);
        tracing::error!(err_msg);
        insert_error_log(err_msg.to_string()).await?;
        // delete all player's games, sets, and tournaments
        delete_games_from_requester_id(set_getter_vars.playerId).await?;
        delete_sets_by_requester_id(set_getter_vars.playerId).await?;
        delete_tournaments_from_requester_id(set_getter_vars.playerId).await?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn execute<T>(_: i32, set_getter_data: T) -> Result<bool>
where
    T: GQLData,
{
    let sgd = set_getter_data.downcast_ref::<SetGetterData>();
    let player = sgd.unwrap().player.clone();

    let mut curated_sets = vec![];
    let mut curated_games = vec![];
    let mut curated_tournaments = vec![];

    let mut db_connection = smithe_database::connect().await?;

    let ss = player.sets.unwrap().nodes;
    // ^^^ guaranteed to have sets in this context, ok to unwrap

    if ss.is_empty() {
        tracing::info!("🏁 finished compiling results for this player!");
        Ok(true)
    } else {
        for s in ss {
            // we only want to compile results for: double elimination single ssbu brackets
            if s.event.is_some()
                && is_ssbu_singles_and_supported_tournament(&s)
                && s.completedAt.is_some()
                && s.event.clone().unwrap().standings.is_some()
                && !s
                    .event
                    .clone()
                    .unwrap()
                    .standings
                    .as_ref()
                    .unwrap()
                    .nodes
                    .is_empty()
            {
                tracing::info!(
                    "🍥 processing set from tourney \"{}\"",
                    s.event.clone().unwrap().tournament.clone().unwrap().name
                );

                let requester_entrant_id = if is_tournament_finished(&s) {
                    get_requester_id_from_standings(&s, player.id)
                } else {
                    continue;
                };

                let maybe_games =
                    maybe_get_games_from_set(player.id, requester_entrant_id, &s, s.id);

                // if there are games, we want to add to the vec to insert in the DB at the end
                if let Some(mut games) = maybe_games.clone() {
                    curated_games.append(&mut games);
                }

                let rslot = get_requester_set_slot(requester_entrant_id, &s);
                let oslot = get_opponent_set_slot(requester_entrant_id, &s);

                if let (Some(r), Some(o)) = (rslot, oslot) {
                    // tournaments could be finished, but not have actually finished
                    // some sets only have a reported winner, ignore them
                    // e.g., https://www.start.gg/tournament/mainstage-2021/event/ultimate-singles/brackets/952392/1513154
                    if let (
                        Some(r_standing_stats),
                        Some(o_standing_stats),
                        Some(s_completed_at),
                        Some(r_seed_num),
                        Some(o_seed_num),
                    ) = (
                        r.standing
                            .as_ref()
                            .and_then(|standing| standing.stats.as_ref())
                            .and_then(|stats| stats.score.value.as_ref()),
                        o.standing
                            .as_ref()
                            .and_then(|standing| standing.stats.as_ref())
                            .and_then(|stats| stats.score.value.as_ref()),
                        s.completedAt,
                        r.seed.as_ref().and_then(|s| s.seedNum),
                        o.seed.as_ref().and_then(|s| s.seedNum),
                    ) {
                        if let (Some(r_entrant_name), Some(o_entrant_name)) = (
                            r.entrant.as_ref().and_then(|entrant| entrant.name.as_ref()),
                            o.entrant.as_ref().and_then(|entrant| entrant.name.as_ref()),
                        ) {
                            if let Some(s_event) = s.event.clone() {
                                if let (Some(is_online), Some(eid), Some(tid)) = (
                                    s_event.isOnline,
                                    s_event.id,
                                    s_event.tournament.as_ref().map(|t| t.id),
                                ) {
                                    curated_sets.push(Set::new(
                                        s.id,
                                        s_completed_at,
                                        player.id,
                                        is_online,
                                        eid,
                                        tid,
                                        r_entrant_name,
                                        *r_standing_stats as i32,
                                        r_seed_num,
                                        o_entrant_name,
                                        *o_standing_stats as i32,
                                        o_seed_num,
                                    ));
                                }
                            }
                        }
                    }

                    let tourney_seed = get_seed(requester_entrant_id, &s);

                    if tourney_seed.is_some() {
                        let tournament = Tournament::new(
                            s.event.clone().unwrap().tournament.as_ref().unwrap().id,
                            s.event.clone().unwrap().id.unwrap(),
                            s.event.clone().unwrap().name.as_ref().unwrap(),
                            &s.event.clone().unwrap().tournament.as_ref().unwrap().name,
                            s.event
                                .clone()
                                .unwrap()
                                .tournament
                                .as_ref()
                                .unwrap()
                                .endAt
                                .unwrap(),
                            player.id,
                            get_placement(&s, player.id),
                            s.event.clone().unwrap().numEntrants.unwrap(),
                            tourney_seed.unwrap(),
                            format!(
                                "https://www.start.gg/{}",
                                s.event.clone().unwrap().slug.as_ref().unwrap()
                            )
                            .as_str(),
                        );

                        if !is_tournament_cached(player.id, &s).await?
                            && !curated_tournaments.contains(&tournament)
                        {
                            // ^^^ not found
                            curated_tournaments.push(tournament);
                        }
                    } else {
                        tracing::info!("🚫 tournament seed not found, skipping...");
                        continue;
                    }
                }
            }
            // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
            // we want to panic regardless
            else {
                if s.event.is_some() {
                    tracing::info!(
                        "🚫 skipping set from tourney \"{}\"",
                        s.event.clone().unwrap().tournament.as_ref().unwrap().name
                    );
                } else {
                    tracing::info!("🚫 skipping set from tourney \"{}\"", "unknown");
                }
                continue;
            }
        }

        tracing::info!("📦 inserting results into db...");
        for g in curated_games {
            let res = insert_into(player_games)
                .values(g)
                .execute(&mut db_connection)
                .await;

            if let Err(e) = res {
                let err_msg = format!("🚨 error inserting game into db: {}", e);
                tracing::error!(err_msg);
                insert_error_log(err_msg.to_string()).await?;
            }
        }

        for s in curated_sets {
            let res = insert_into(player_sets)
                .values(s)
                .execute(&mut db_connection)
                .await;

            if let Err(e) = res {
                let err_msg = format!("🚨 error inserting set into db: {}", e);
                tracing::error!(err_msg);
                insert_error_log(err_msg.to_string()).await?;
            }
        }

        for t in curated_tournaments {
            let res = insert_into(player_tournaments)
                .values(t)
                .execute(&mut db_connection)
                .await;

            if let Err(e) = res {
                let err_msg = format!("🚨 error inserting tournament into db: {}", e);
                tracing::error!(err_msg);
                insert_error_log(err_msg.to_string()).await?;
            }
        }

        Ok(false)
    }
}

// get player's top two most played characters
pub async fn get_top_two_characters(pid: i32) -> Result<Vec<Option<String>>> {
    let mut db_connection = smithe_database::connect().await?;

    let top_two_characters = player_games
        .select(requester_char_played)
        .filter(smithe_database::schema::player_games::requester_id.eq(pid))
        .group_by(requester_char_played)
        .order_by(count(requester_char_played).desc())
        .limit(2)
        .get_results::<Option<String>>(&mut db_connection)
        .await?;

    Ok(top_two_characters)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use diesel_async::scoped_futures::ScopedFutureExt;
    use diesel_async::AsyncConnection;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;
    const DANTOTTO_PLAYER_SLUG: &str = "566b1fb5";

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_check_if_large_consecutive_playerid_grouping_exists() {
        let res = check_if_large_consecutive_playerid_grouping_exists().await;
        assert!(res.is_ok());
        assert!(!res.unwrap());
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_max_player_id() {
        let res = get_max_player_id().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_top_two_characters() {
        let res = get_top_two_characters(DANTOTTO_PLAYER_ID).await;
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![Some("King Dedede".to_string()), Some("Steve".to_string())]
        );
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_subsequent_player_id_with_circle_back() {
        let res = get_subsequent_player_id_with_circle_back(DANTOTTO_PLAYER_ID).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1178566);
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_empty_user_with_slug() {
        let res = get_empty_user_with_slug(DANTOTTO_PLAYER_ID).await;
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().unwrap().slug,
            Some("user/566b1fb5".to_string())
        );
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_add_new_empty_player_record() -> Result<()> {
        let mut db_connection = smithe_database::connect().await?;
        let err = db_connection
            .transaction::<(), _, _>(|db_connection| {
                async {
                    add_new_empty_player_record_provided_connection(
                        DANTOTTO_PLAYER_ID,
                        db_connection,
                    )
                    .await
                    .expect("failed to add new empty player record");

                    // check that the player id was added
                    assert_eq!(
                        empty_player_ids
                            .filter(
                                smithe_database::schema::empty_player_ids::player_id
                                    .eq(DANTOTTO_PLAYER_ID)
                            )
                            .count()
                            .get_result::<i64>(db_connection)
                            .await?,
                        1
                    );

                    Err(diesel::result::Error::RollbackTransaction)
                }
                .scope_boxed()
            })
            .await;

        assert!(err.is_err());

        // check no id in empty_player_ids
        assert_eq!(
            empty_player_ids
                .filter(smithe_database::schema::empty_player_ids::player_id.eq(DANTOTTO_PLAYER_ID))
                .count()
                .get_result::<i64>(&mut db_connection)
                .await?,
            0
        );

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_add_new_player_to_pidgtm_db() -> Result<()> {
        let mut db_connection = smithe_database::connect().await?;
        let err = db_connection
            .transaction::<(), _, _>(|db_connection| {
                async {
                    add_new_player_to_pidgtm_db_provided_connection(
                        &SGGPlayer {
                            id: 999,
                            prefix: None,
                            gamerTag: Some("Orinorae Thaamtekelud".to_string()),
                            rankings: None,
                            user: Some(User {
                                name: None,
                                location: None,
                                bio: None,
                                birthday: None,
                                images: None,
                                slug: Some("user/123a4bc5".to_string()),
                                genderPronoun: None,
                                authorizations: None,
                            }),
                            sets: None,
                        },
                        db_connection,
                    )
                    .await
                    .expect("failed to add new player to pidgtm db");

                    // check that the player id was added
                    assert_eq!(
                        players
                            .filter(smithe_database::schema::players::player_id.eq(999))
                            .count()
                            .get_result::<i64>(db_connection)
                            .await?,
                        1
                    );

                    Err(diesel::result::Error::RollbackTransaction)
                }
                .scope_boxed()
            })
            .await;

        assert!(err.is_err());

        // check no id in empty_player_ids
        assert_eq!(
            players
                .filter(smithe_database::schema::players::player_id.eq(999))
                .count()
                .get_result::<i64>(&mut db_connection)
                .await?,
            0
        );

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_all_like() {
        let res = get_all_like("dantotto").await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1);
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_player() {
        let res = get_player(DANTOTTO_PLAYER_ID).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().player_id, DANTOTTO_PLAYER_ID);
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_player_from_slug() {
        let res = get_player_from_slug(DANTOTTO_PLAYER_SLUG).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().player_id, DANTOTTO_PLAYER_ID);
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_highest_id_with_sets_between() {
        let res = get_highest_id_with_sets_between(1000, 1001).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Some(1000));
    }
}
