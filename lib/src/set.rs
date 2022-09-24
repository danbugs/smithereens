use anyhow::Result;
use diesel::{dsl::sql, prelude::*};

use smithe_database::{db_models::set::Set, schema::player_sets::dsl::*};

use startgg::{Set as SGGSet, SetSlot as SGGSetSlot};

pub fn get_all_from_player_id(player_id: i32) -> Result<Vec<Set>> {
    let db_connection = smithe_database::connect()?;
    let cache = player_sets
        .filter(requester_id.eq(player_id))
        .load::<Set>(&db_connection)?;

    Ok(cache)
}

pub fn get_last_completed_at(cache: Vec<Set>) -> Option<i64> {
    if !cache.is_empty() {
        tracing::info!("✅ player was cached...");
        Some(
            cache
                .iter()
                .max_by_key(|s| s.completed_at)
                .unwrap()
                .completed_at
                + 1,
        )
    } else {
        tracing::info!("❌ player was not cached...");
        None
    }
}

/// Provides a set with access to:
/// - entrant name,
/// - entrant seed, and
/// - entrant set score (e.g., won 2 games, DQd, etc.).
pub fn get_requester_set_slot(requester_entrant_id: i32, s: &SGGSet) -> SGGSetSlot {
    s.slots
        .iter()
        .find(|i| i.entrant.id.as_ref().unwrap().eq(&requester_entrant_id))
        .unwrap()
        .clone()
}

/// Provides a set with access to:
/// - entrant name,
/// - entrant seed, and
/// - entrant set score (e.g., won 2 games, DQd, etc.).
pub fn get_opponent_set_slot(requester_entrant_id: i32, s: &SGGSet) -> SGGSetSlot {
    s.slots
        .iter()
        .find(|i| i.entrant.id.as_ref().unwrap().ne(&requester_entrant_id))
        .unwrap()
        .clone()
}

pub fn get_set_wins_without_dqs(player_id: i32) -> Result<i64> {
    let db_connection = smithe_database::connect()?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(2))
        .count()
        .get_result::<i64>(&db_connection)?)
}

pub fn get_set_losses_without_dqs(player_id: i32) -> Result<i64> {
    let db_connection = smithe_database::connect()?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(-2))
        .count()
        .get_result::<i64>(&db_connection)?)
}

pub fn get_set_wins_by_dq(player_id: i32) -> Result<i64> {
    let db_connection = smithe_database::connect()?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(1))
        .count()
        .get_result::<i64>(&db_connection)?)
}

pub fn get_set_losses_by_dq(player_id: i32) -> Result<i64> {
    let db_connection = smithe_database::connect()?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(-1))
        .count()
        .get_result::<i64>(&db_connection)?)
}

pub fn get_winrate(player_id: i32) -> Result<f32> {
    let set_wins_without_dqs = get_set_wins_without_dqs(player_id)?;
    let set_losses_without_dqs = get_set_losses_without_dqs(player_id)?;
    Ok(
        ((set_wins_without_dqs as f32) / ((set_wins_without_dqs + set_losses_without_dqs) as f32))
            .abs()
            * 100.0,
    )
}

pub fn get_competitor_type(player_id: i32) -> Result<(u32, u32)> {
    let db_connection = smithe_database::connect()?;
    let raw_player_results = player_sets
        .filter(requester_id.eq(player_id))
        .group_by(event_id)
        .select((
            event_id,
            sql("COUNT(result_type>0 OR NULL)"),
            sql("COUNT(result_type<0 OR NULL)"),
        ))
        .get_results::<(i32, String, String)>(&db_connection)?;
    // ^^^ not sure why but have to get the count as text

    let player_results = raw_player_results
        .iter()
        .map(|i| {
            (
                i.0,
                i.1.chars().nth_back(0).unwrap() as u32,
                i.2.chars().nth_back(0).unwrap() as u32,
            )
        })
        .collect::<Vec<(i32, u32, u32)>>();

    Ok((
        ((player_results.iter().map(|i| i.1).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
        ((player_results.iter().map(|i| i.2).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
    ))
}
