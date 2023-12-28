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

use diesel::{
    delete,
    dsl::{count, max},
    insert_into,
    prelude::*,
    sql_query, update,
};
use smithe_database::{
    db_models::{empty_player_ids::EmptyPlayerId, last_checked_player_id::LastCheckedPlayerId},
    schema::last_checked_player_id,
    schema::{empty_player_ids::dsl::*, last_checked_player_id::dsl::*},
};
use startgg::{queries::set_getter::SetGetterData, GQLData, Player as SGGPlayer, User};

use crate::{
    game::maybe_get_games_from_set,
    set::{get_opponent_set_slot, get_requester_set_slot},
    tournament::{
        get_placement, get_requester_id_from_standings, get_seed,
        is_ssbu_singles_double_elimination_tournament, is_tournament_cached,
        is_tournament_finished,
    },
};

pub fn get_all_like(tag: &str) -> Result<Vec<Player>> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    let db_connection = smithe_database::connect()?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag.ilike(format!("%{}%", processed_tag))) // case-insensitive like
        .get_results::<Player>(&db_connection)?;

    Ok(matching_players)
}

pub fn get_player(pid: i32) -> Result<Player> {
    let db_connection = smithe_database::connect()?;
    let matched_player = players
        .filter(smithe_database::schema::players::player_id.eq(pid)) // case-insensitive like
        .get_result::<Player>(&db_connection)?;

    Ok(matched_player)
}

pub fn get_last_cached_player_id() -> Result<i32> {
    let db_connection = smithe_database::connect()?;
    let max_checked_player_id = last_checked_player_id
        .select(max(last_checked_player_id::player_id))
        .first::<Option<i32>>(&db_connection)?;
    if let Some(val) = max_checked_player_id {
        Ok(val)
    } else {
        // return max player id from players table
        let max_player_id = players
            .select(max(smithe_database::schema::players::player_id))
            .first::<Option<i32>>(&db_connection)?;
        if let Some(val) = max_player_id {
            Ok(val)
        } else {
            Ok(1000) // nothing in db, start at 1000
        }
    }
}

pub fn increment_last_cached_player_id(curr_player_id: i32) -> Result<()> {
    let db_connection = smithe_database::connect()?;
    insert_into(last_checked_player_id)
        .values(LastCheckedPlayerId::from(curr_player_id))
        .execute(&db_connection)?;

    Ok(())
}

pub fn add_new_player_to_pidgtm_db(pti: &SGGPlayer) -> Result<()> {
    let db_connection = smithe_database::connect()?;
    insert_into(players)
        .values(Player::from(pti.clone()))
        .execute(&db_connection)?;
    Ok(())
}

pub fn update_player_in_pidgtm_db(pti: &SGGPlayer) -> Result<()> {
    let db_connection = smithe_database::connect()?;
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
        .execute(&db_connection)?;
    Ok(())
}

pub fn add_new_empty_player_record(pid: i32) -> Result<()> {
    let db_connection = smithe_database::connect()?;
    insert_into(empty_player_ids)
        .values(EmptyPlayerId::from(pid))
        .execute(&db_connection)?;
    Ok(())
}

pub fn get_subsequent_player_id_with_circle_back(some_id: i32) -> Result<i32> {
    let db_connection = smithe_database::connect()?;
    let res = players
        .select(smithe_database::schema::players::player_id)
        .filter(smithe_database::schema::players::player_id.gt(some_id))
        .order(smithe_database::schema::players::player_id.asc())
        .first(&db_connection)
        .optional()?;

    if let Some(r) = res {
        Ok(r)
    } else {
        Ok(1000) // circle back logic
    }
}

pub fn check_if_large_consecutive_playerid_grouping_exists() -> Result<bool> {
    let db_connection = smithe_database::connect()?;
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
    .load(&db_connection)?; // 1144 is the number of players in the largest consecutive grouping

    // if vec is not empty, then there is a large consecutive grouping
    Ok(!res.is_empty())
}

pub fn delete_large_consecutive_playerid_grouping() -> Result<()> {
    let db_connection = smithe_database::connect()?;
    let max_player_id = get_max_player_id()?;

    // delete all empty_player_ids that are greater than the max player id
    delete(empty_player_ids)
        .filter(smithe_database::schema::empty_player_ids::player_id.gt(max_player_id))
        .execute(&db_connection)?;

    Ok(())
}

pub fn get_max_player_id() -> Result<i32> {
    let db_connection = smithe_database::connect()?;
    let max_player_id = players
        .select(max(smithe_database::schema::players::player_id))
        .first::<Option<i32>>(&db_connection)?;
    if let Some(val) = max_player_id {
        Ok(val)
    } else {
        Ok(1000) // nothing in db, start at 1000
    }
}

pub fn get_subsequent_player_id_without_circle_back(some_id: i32) -> Result<Option<i32>> {
    let db_connection = smithe_database::connect()?;
    Ok(players
        .select(smithe_database::schema::players::player_id)
        .filter(smithe_database::schema::players::player_id.gt(some_id))
        .order(smithe_database::schema::players::player_id.asc())
        .first(&db_connection)
        .optional()?)
}
pub fn get_empty_user_with_slug(pid: i32) -> Result<Option<User>> {
    let db_connection = smithe_database::connect()?;
    let some_slug = players
        .select(user_slug)
        .filter(smithe_database::schema::players::player_id.eq(pid))
        .get_result(&db_connection)
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

pub fn execute<T>(_: i32, set_getter_data: T) -> Result<bool>
where
    T: GQLData,
{
    let sgd = set_getter_data.downcast_ref::<SetGetterData>();
    let player = sgd.unwrap().player.clone();

    let mut curated_sets = vec![];
    let mut curated_games = vec![];
    let mut curated_tournaments = vec![];

    let db_connection = smithe_database::connect()?;

    let ss = player.sets.unwrap().nodes;
    // ^^^ guaranteed to have sets in this context, ok to unwrap

    if ss.is_empty() {
        tracing::info!("🏁 finished compiling results for this player!");
        Ok(true)
    } else {
        tracing::info!("✅ got some results...");
        for s in ss {
            tracing::info!(
                "🍥 processing set from tourney \"{}\"",
                s.event.tournament.clone().unwrap().name
            );

            // we only want to compile results for: double elimination single ssbu brackets
            if is_ssbu_singles_double_elimination_tournament(&s) && s.completedAt.is_some() {
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
                    if r.standing
                        .as_ref()
                        .unwrap()
                        .stats
                        .as_ref()
                        .unwrap()
                        .score
                        .value
                        .is_some()
                        && o.standing
                            .as_ref()
                            .unwrap()
                            .stats
                            .as_ref()
                            .unwrap()
                            .score
                            .value
                            .is_some()
                        && s.completedAt.is_some()
                    {
                        curated_sets.push(Set::new(
                            s.id,
                            s.completedAt.unwrap(),
                            player.id,
                            s.event.isOnline.unwrap(),
                            s.event.id.unwrap(),
                            s.event.tournament.as_ref().unwrap().id,
                            r.entrant.as_ref().unwrap().name.as_ref().unwrap(),
                            r.standing
                                .as_ref()
                                .unwrap()
                                .stats
                                .as_ref()
                                .unwrap()
                                .score
                                .value
                                .unwrap(),
                            r.seed.as_ref().unwrap().seedNum,
                            o.entrant.as_ref().unwrap().name.as_ref().unwrap(),
                            o.standing
                                .as_ref()
                                .unwrap()
                                .stats
                                .as_ref()
                                .unwrap()
                                .score
                                .value
                                .unwrap(),
                            o.seed.as_ref().unwrap().seedNum,
                        ));
                    }

                    let tournament = Tournament::new(
                        s.event.tournament.as_ref().unwrap().id,
                        s.event.id.unwrap(),
                        s.event.name.as_ref().unwrap(),
                        &s.event.tournament.as_ref().unwrap().name,
                        s.event.tournament.as_ref().unwrap().endAt,
                        player.id,
                        get_placement(&s, player.id),
                        s.event.numEntrants.unwrap(),
                        get_seed(requester_entrant_id, &s),
                        format!("https://www.start.gg/{}", s.event.slug.as_ref().unwrap()).as_str(),
                    );

                    if !is_tournament_cached(player.id, &s)?
                        && !curated_tournaments.contains(&tournament)
                    {
                        // ^^^ not found
                        curated_tournaments.push(tournament);
                    }
                }
            }
            // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
            // we want to panic regardless
        }

        for g in curated_games {
            let res = insert_into(player_games).values(g).execute(&db_connection);

            if let Err(e) = res {
                tracing::error!("🚨 error inserting game into db: {}", e);
            }
        }

        for s in curated_sets {
            let res = insert_into(player_sets).values(s).execute(&db_connection);

            if let Err(e) = res {
                tracing::error!("🚨 error inserting set into db: {}", e);
            }
        }

        for t in curated_tournaments {
            let res = insert_into(player_tournaments)
                .values(t)
                .execute(&db_connection);

            if let Err(e) = res {
                tracing::error!("🚨 error inserting tournament into db: {}", e);
            }
        }

        Ok(false)
    }
}

// get player's top two most played characters
pub fn get_top_two_characters(pid: i32) -> Result<Vec<Option<String>>> {
    let db_connection = smithe_database::connect()?;

    let top_two_characters = player_games
        .select(requester_char_played)
        .filter(smithe_database::schema::player_games::requester_id.eq(pid))
        .group_by(requester_char_played)
        .order_by(count(requester_char_played).desc())
        .limit(2)
        .get_results::<Option<String>>(&db_connection)?;

    Ok(top_two_characters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_large_consecutive_playerid_grouping_exists() {
        check_if_large_consecutive_playerid_grouping_exists().unwrap();
    }
}
