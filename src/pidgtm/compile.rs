use anyhow::Result;
use smithe_lib::{
    player::get_subsequent_player_id_without_circle_back,
    tournament::get_tournaments_from_requester_id,
};

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
        tracing::info!("🧪 compiling player (id: '{:#?}')...", rid);
        get_tournaments_from_requester_id(rid.unwrap_or(1000)).await?;

        rid = get_subsequent_player_id_without_circle_back(rid.unwrap())?;
    }

    Ok(())
}
