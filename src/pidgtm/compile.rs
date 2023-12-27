use anyhow::Result;
use smithe_lib::{
    player::get_subsequent_player_id_without_circle_back,
    tournament::get_tournaments_from_requester_id,
};

pub async fn handle_compile(start_at_player_id: Option<i32>) -> Result<()> {
    let mut rid = start_at_player_id.unwrap_or(1000);
    loop {
        let _tournaments = get_tournaments_from_requester_id(rid).await?;

        if let Some(r) = get_subsequent_player_id_without_circle_back(rid)? {
            rid = r;
        } else {
            break;
        }
    }

    Ok(())
}
