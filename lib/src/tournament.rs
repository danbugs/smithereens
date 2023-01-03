use std::sync::{Arc, Mutex};

use anyhow::Result;
use diesel::prelude::*;

use smithe_database::{db_models::tournament::Tournament, schema::player_tournaments::dsl::*};
use startgg::{Set as SGGSet, queries::set_getter::{SetGetterVars, make_set_getter_query}};

use crate::{set::{get_all_from_player_id, get_last_completed_at}, player::{get_player, execute}, common::start_read_all_by_increment_execute_finish_maybe_cancel};

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

// get tournaments from requester id
pub async fn get_tournaments_from_requester_id(rid: i32) -> Result<Vec<Tournament>> {
    // get player from pid
    let p = get_player(rid)?;

    let cache = get_all_from_player_id(p.player_id)?;

    let updated_after = get_last_completed_at(cache);
    let usgv = SetGetterVars::unpaginated_new(
        p.player_id,
        updated_after,
        &p.gamer_tag,
    );

    start_read_all_by_increment_execute_finish_maybe_cancel(
        false,
        Arc::new(Mutex::new(usgv)),
        make_set_getter_query,
        1,
        execute,
        |curr_page| Ok(curr_page + 1),
        |_| Ok(()),
        |_| Ok(()),
    )
    .await?;

    let db_connection = smithe_database::connect()?;
    let tournaments = player_tournaments
        .filter(requester_id.eq(rid))
        .get_results::<Tournament>(&db_connection)?;
    
    Ok(tournaments)
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
