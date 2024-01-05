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
        None,
        execute,
        |curr_page| Ok(curr_page + 1),
        |_| Ok(()),
        |_| Ok(()),
    )
    .await?;

    let mut db_connection = smithe_database::connect()?;
    let tournaments = player_tournaments
        .filter(requester_id.eq(rid))
        .get_results::<Tournament>(&mut db_connection)?;

    Ok(tournaments)
}

pub fn is_ssbu_singles_and_supported_tournament(s: &SGGSet) -> bool {
    s.event.clone().unwrap().videogame.as_ref().unwrap().name == "Super Smash Bros. Ultimate"
        && s.phaseGroup.is_some()
        && (s.phaseGroup.clone().unwrap().bracketType == "DOUBLE_ELIMINATION"
            || s.phaseGroup.clone().unwrap().bracketType == "SINGLE_ELIMINATION"
            || s.phaseGroup.clone().unwrap().bracketType == "ROUND_ROBIN"
            || s.phaseGroup.clone().unwrap().bracketType == "SWISS")
        && s.event.clone().unwrap().teamRosterSize.is_none()
}

pub fn get_num_tournaments_attended(pid: i32) -> Result<i64> {
    let mut db_connection = smithe_database::connect()?;
    get_num_tournaments_attended_provided_connection(pid, &mut db_connection)
}

fn get_num_tournaments_attended_provided_connection(
    pid: i32,
    db_connection: &mut PgConnection,
) -> Result<i64> {
    let count: i64 = player_tournaments
        .select(count_star())
        .filter(requester_id.eq(pid))
        .first(db_connection)?;

    Ok(count)
}

pub fn get_seed(requester_entrant_id: i32, s: &SGGSet) -> Option<i32> {
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
    let mut db_connection = smithe_database::connect()?;
    Ok(player_tournaments
        .find((
            s.event.clone().unwrap().tournament.as_ref().unwrap().id,
            s.event.clone().unwrap().id.unwrap(),
            player_id,
        ))
        .first::<Tournament>(&mut db_connection)
        .is_ok())
}

// delete a player's tournaments given a requester id
pub fn delete_tournaments_from_requester_id(player_id: i32) -> Result<()> {
    let mut db_connection = smithe_database::connect()?;
    delete_tournaments_from_requester_id_provided_connection(player_id, &mut db_connection)
}

fn delete_tournaments_from_requester_id_provided_connection(
    player_id: i32,
    db_connection: &mut PgConnection,
) -> Result<()> {
    diesel::delete(player_tournaments.filter(requester_id.eq(player_id)))
        .execute(db_connection)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use diesel::Connection;

    use crate::common::{init_logger, get_sggset_test_data};

    const DANTOTTO_PLAYER_ID: i32 = 1178271;
    const TYRESE_PLAYER_ID: i32 = 2021528;

    #[tokio::test]
    async fn get_tournaments_from_requester_id_test() -> Result<()> {
        init_logger()?;
        let now = std::time::Instant::now();
        let _ = super::get_tournaments_from_requester_id(DANTOTTO_PLAYER_ID).await?;
        let elapsed = now.elapsed();

        tracing::info!("Test took: {:?} seconds", elapsed.as_secs());
        Ok(())
    }

    #[test]
    fn is_tournament_finished_test() {
        let s = get_sggset_test_data();

        assert_eq!(super::is_tournament_finished(&s), true);
    }

    #[test]
    fn get_placement_test() {
        let s = get_sggset_test_data();

        assert_eq!(super::get_placement(&s, TYRESE_PLAYER_ID), 5);
    }

    #[test]
    fn get_requester_id_from_standings_test() {
        let s = get_sggset_test_data();

        assert_eq!(super::get_requester_id_from_standings(&s, TYRESE_PLAYER_ID), 9410060);
    }

    #[test]
    fn is_ssbu_singles_and_supported_tournament_test() {
        let s = get_sggset_test_data();

        assert_eq!(super::is_ssbu_singles_and_supported_tournament(&s), true);
    }

    #[test]
    fn get_num_tournaments_attended_test() -> Result<()> {
        let count = super::get_num_tournaments_attended(DANTOTTO_PLAYER_ID)?;

        // check that it is greater than or equalt to 237
        assert!(count >= 237);

        Ok(())
    }

    #[test]
    fn get_seed_test() {
        let s = get_sggset_test_data();

        assert_eq!(super::get_seed(9410060, &s), Some(10));
    }

    #[test]
    fn is_tournament_cached_test() -> Result<()> {
        let s = get_sggset_test_data();

        assert_eq!(
            super::is_tournament_cached(TYRESE_PLAYER_ID, &s)?,
            true
        );

        Ok(())
    }

    #[test]
    fn delete_tournaments_from_requester_id_test() -> Result<()> {
        let mut db_connection = smithe_database::connect()?;
        
        let err = db_connection.transaction::<(),_ , _>(|db_connection| {
            super::delete_tournaments_from_requester_id_provided_connection(DANTOTTO_PLAYER_ID, db_connection).expect("failed to delete tournaments");

            // check player doesn't have any tournaments
            assert_eq!(
                super::get_num_tournaments_attended_provided_connection(DANTOTTO_PLAYER_ID, db_connection).expect("failed to get num tournaments"),
                0
            );            

            Err(diesel::result::Error::RollbackTransaction)
        });

        assert!(err.is_err());

        // check player has tournaments again
        assert!(
            super::get_num_tournaments_attended(DANTOTTO_PLAYER_ID)? > 0
        );

        Ok(())
    }
}
