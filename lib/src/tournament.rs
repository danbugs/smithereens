use std::sync::{Arc, Mutex};

use anyhow::Result;
use diesel::{dsl::count_star, prelude::*};

use smithe_database::{db_models::tournament::Tournament, schema::player_tournaments::dsl::*};
use startgg::{
    queries::set_getter::{make_set_getter_query, SetGetterVars},
    Set as SGGSet, Standing,
};

use crate::{
    common::start_read_all_by_increment_execute_finish_maybe_cancel,
    player::{execute, get_player},
    set::{get_all_from_player_id, get_last_completed_at},
};

pub fn is_tournament_finished(s: &SGGSet) -> bool {
    let e_at = s.event.clone().unwrap().tournament.as_ref().unwrap().endAt;
    e_at.is_some() && (e_at.unwrap() <= chrono::Utc::now().timestamp())
}

fn get_standing_of_player_from_sggset(s: &SGGSet, player_id: i32) -> Standing {
    let standing_nodes = s
        .event
        .clone()
        .unwrap()
        .standings
        .as_ref()
        .unwrap()
        .nodes
        .clone();

    if let Some(a) = standing_nodes
        .iter()
        .find(|i| i.player.as_ref().unwrap().id.eq(&player_id))
    {
        a.clone()
    } else {
        // if the player id is not matched in the standings, they are anonymous
        // in this case, we just return the first match off of the gamer tag query
        // this is not ideal because, if there are two players with the same tag,
        // it could incorrectly match them to the wrong player.
        standing_nodes[0].clone()
    }
}

pub fn get_placement(s: &SGGSet, player_id: i32) -> i32 {
    let standing = get_standing_of_player_from_sggset(s, player_id);
    standing.placement.unwrap()
}

pub fn get_requester_id_from_standings(s: &SGGSet, player_id: i32) -> i32 {
    let standing = get_standing_of_player_from_sggset(s, player_id);
    standing.entrant.unwrap().id.unwrap()
}

// get tournaments from requester id
pub async fn get_tournaments_from_requester_id(rid: i32) -> Result<Vec<Tournament>> {
    tracing::info!("getting tournaments from requester id: {}", rid);

    // get player from pid
    let p = get_player(rid)?;

    let cache = get_all_from_player_id(p.player_id)?;

    let updated_after = get_last_completed_at(cache);
    let usgv = SetGetterVars::unpaginated_new(p.player_id, updated_after, &p.gamer_tag);

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
    s.event.clone().unwrap().videogame.as_ref().unwrap().name == "Super Smash Bros. Ultimate"
        && s.phaseGroup.is_some()
        && s.phaseGroup.clone().unwrap().bracketType == "DOUBLE_ELIMINATION"
        && s.event.clone().unwrap().teamRosterSize.is_none()
}

pub fn get_num_tournaments_attended(pid: i32) -> Result<i64> {
    let db_connection = smithe_database::connect()?;
    let count: i64 = player_tournaments
        .select(count_star())
        .filter(requester_id.eq(pid))
        .first(&db_connection)?;

    Ok(count)
}

pub fn get_seed(requester_entrant_id: i32, s: &SGGSet) -> i32 {
    s.slots
        .iter()
        .find(|i| {
            i.entrant
                .as_ref()
                .unwrap()
                .id
                .as_ref()
                .unwrap()
                .eq(&requester_entrant_id)
        })
        .unwrap()
        .seed
        .as_ref()
        .unwrap()
        .seedNum
}

pub fn is_tournament_cached(player_id: i32, s: &SGGSet) -> Result<bool> {
    let db_connection = smithe_database::connect()?;
    Ok(player_tournaments
        .find((
            s.event.clone().unwrap().tournament.as_ref().unwrap().id,
            s.event.clone().unwrap().id.unwrap(),
            player_id,
        ))
        .first::<Tournament>(&db_connection)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::common::init_logger;

    const HUNGRYBOX_PLAYER_ID: i32 = 1004;

    #[tokio::test]
    async fn get_tournaments_from_requester_id_test() -> Result<()> {
        init_logger()?;
        let _ = super::get_tournaments_from_requester_id(HUNGRYBOX_PLAYER_ID).await?;
        Ok(())
    }
}
