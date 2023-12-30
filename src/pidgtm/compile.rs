use anyhow::Result;
use smithe_lib::tournament::get_tournaments_from_requester_id;

use super::map::{map_increment, map_operation};

pub async fn handle_compile(
    start_at_player_id: Option<i32>,
    end_at_player_id: Option<i32>,
) -> Result<()> {
    let mut rid = start_at_player_id;

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
            rid = Some(map_increment(rid.unwrap())?);
        } else if res.is_err() {
            // any other error is a problem
            panic!("Error: {:?}", res);
        } else {
            rid = Some(map_increment(rid.unwrap())?);
        }
    }

    tracing::info!("🏁 finished compiling player data to pidgtm db");

    Ok(())
}
