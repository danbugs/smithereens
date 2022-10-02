use std::sync::{Arc, Mutex};

use anyhow::Result;

use diesel::{dsl::max, insert_into, prelude::*, update};
use smithe_database::{
    db_models::{
        empty_player_ids::EmptyPlayerId, last_checked_player_id::LastCheckedPlayerId,
        player::Player,
    },
    schema::last_checked_player_id,
    schema::players::dsl::*,
    schema::{empty_player_ids::dsl::*, last_checked_player_id::dsl::*},
};
use startgg::{queries::player_getter::PIDGTM_PlayerGetterVars, Player as SGGPlayer};

pub fn get_all_like(tag: &str) -> Result<Vec<Player>> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    let db_connection = smithe_database::connect()?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag.ilike(format!("%{}%", processed_tag))) // case-insensitive like
        .get_results::<Player>(&db_connection)?;

    Ok(matching_players)
}

pub fn get_last_cached_player_id() -> Result<i32> {
    let db_connection = smithe_database::connect()?;
    let max_checked_player_id = last_checked_player_id
        .select(max(last_checked_player_id::player_id))
        .first::<Option<i32>>(&db_connection)?;
    if let Some(val) = max_checked_player_id {
        Ok(val)
    } else {
        Ok(1)
        // ^^^ don't start at 0 because it is uniquely populated
        // w/ all null values but non null player
    }
}

pub fn increment_last_cached_player_id(pgv: Arc<Mutex<PIDGTM_PlayerGetterVars>>) -> Result<()> {
    let db_connection = smithe_database::connect()?;
    insert_into(last_checked_player_id)
        .values(LastCheckedPlayerId::from(pgv.lock().unwrap().playerId))
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
