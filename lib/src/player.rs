use std::sync::{Arc, Mutex};

use anyhow::Result;

use diesel::{dsl::max, insert_into, prelude::*};
use smithe_database::{
    db_models::{last_checked_player_id::LastCheckedPlayerId, player::Player},
    schema::last_checked_player_id,
    schema::last_checked_player_id::dsl::*,
    schema::players::dsl::*,
};
use startgg::queries::player_getter::PIDGTM_PlayerGetterVars;

pub fn get_all_like(tag: &str) -> Result<Vec<Player>> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    let db_connection = smithe_database::connect()?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag_with_prefix.ilike(format!("%{}%", processed_tag))) // case-insensitive like
        .get_results::<Player>(&db_connection)?;

    Ok(matching_players)
}

pub fn maybe_remove_prefix_from_gamer_tag(player: &Player) -> String {
    if player.gamer_tag_with_prefix.contains(" | ") {
        player.gamer_tag_with_prefix[player.gamer_tag_with_prefix.find(" | ").unwrap() + 3..]
            .to_string()
    } else {
        player.gamer_tag_with_prefix.clone()
    }
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
