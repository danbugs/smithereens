use anyhow::Result;
use diesel::prelude::*;

use smithe_database::{db_models::tournament::Tournament, schema::player_tournaments::dsl::*};
use startgg::Set as SGGSet;

pub fn is_tournament_finished(s: &SGGSet) -> bool {
    s.event.standings.is_some() && !s.event.standings.as_ref().unwrap().nodes.is_empty()
}

pub fn get_requester_id_from_standings(s: &SGGSet, player_id: i32) -> i32 {
    s.event
        .standings
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|i| i.player.as_ref().unwrap().id.eq(&player_id))
        .unwrap()
        .entrant
        .as_ref()
        .unwrap()
        .id
        .unwrap()
}

pub fn is_ssbu_singles_double_elimination_tournament(s: &SGGSet) -> bool {
    s.event.videogame.as_ref().unwrap().name == "Super Smash Bros. Ultimate"
        && s.phaseGroup.bracketType == "DOUBLE_ELIMINATION"
        && s.event.teamRosterSize.is_none()
}

pub fn get_placement(player_id: i32, s: &SGGSet) -> i32 {
    s.event
        .standings
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|i| i.player.as_ref().unwrap().id.eq(&player_id))
        .unwrap()
        .placement
        .unwrap()
}

pub fn get_seed(requester_entrant_id: i32, s: &SGGSet) -> i32 {
    s.slots
        .iter()
        .find(|i| i.entrant.id.as_ref().unwrap().eq(&requester_entrant_id))
        .unwrap()
        .seed
        .seedNum
}

pub fn is_tournament_cached(player_id: i32, s: &SGGSet) -> Result<bool> {
    let db_connection = smithe_database::connect()?;
    Ok(player_tournaments
        .find((
            s.event.tournament.as_ref().unwrap().id,
            s.event.id.unwrap(),
            player_id,
        ))
        .first::<Tournament>(&db_connection)
        .is_ok())
}
