use anyhow::Result;
use smithe_lib::{
    pidgtm_compile_times::insert_pidgtm_compile_time, player::get_highest_id_with_sets_between,
    tournament::get_tournaments_from_requester_id,
};

use super::map::{map_increment, map_operation};

pub async fn handle_compile(
    start_at_player_id: Option<i32>,
    end_at_player_id: Option<i32>,
) -> Result<()> {
    // if start and end are some, try to get highest id between them
    let mut rid = if start_at_player_id.is_some() && end_at_player_id.is_some() {
        let highest_id = get_highest_id_with_sets_between(
            start_at_player_id.unwrap(),
            end_at_player_id.unwrap(),
        ).await?;

        if let Some(highest_id) = highest_id {
            tracing::info!(
                "📈 discovered cache! Highest id between {} and {} with data is {}",
                start_at_player_id.unwrap(),
                end_at_player_id.unwrap(),
                highest_id
            );
            Some(highest_id)
        } else {
            start_at_player_id
        }
    } else {
        start_at_player_id
    };

    // set end_at_player_id to None if it is less than or equal start
    let end_at_player_id = if end_at_player_id.is_some()
        && end_at_player_id.unwrap() <= start_at_player_id.unwrap_or(1000)
    {
        None
    } else {
        end_at_player_id
    };

    // loop while rid < end_at_player_id, or until rid is None
    while rid.is_some() && end_at_player_id.map(|e| rid.unwrap() < e).unwrap_or(true) {
        // start timer
        let start = std::time::Instant::now();

        map_operation(rid.unwrap(), Some(rid.unwrap() + 1)).await?; // essentially requesting to map 1 player

        let res = get_tournaments_from_requester_id(rid.unwrap_or(1000)).await;

        let map_didnt_add = res.is_err()
            && res
                .as_ref()
                .unwrap_err()
                .to_string()
                .contains("Record not found");
        if map_didnt_add {
            // res could be "Error: Record not found", meaning that ID doesn't belong to a player, if so, continue.
            tracing::info!(
                "⛔ record not found for player id: {}, moving on...",
                rid.unwrap()
            );
            rid = Some(map_increment(rid.unwrap()).await?);
        } else if res.is_err() {
            // any other error is a problem
            panic!("Error: {:?}", res);
        } else {
            rid = Some(map_increment(rid.unwrap()).await?);
        }

        // end timer
        let elapsed = start.elapsed();

        // get time in seconds
        let tis = elapsed.as_secs();
        insert_pidgtm_compile_time(tis as i32).await?; // insert time into db
    }

    tracing::info!("🏁 finished compiling player data to pidgtm db");

    Ok(())
}
