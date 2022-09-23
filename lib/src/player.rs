use anyhow::Result;

use diesel::prelude::*;
use smithe_database::{db_models::player::Player, schema::players::dsl::*};

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
