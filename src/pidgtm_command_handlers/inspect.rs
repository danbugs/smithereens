use anyhow::Result;

use crate::queries::player_getter::make_pidgtm_player_getter_query;

pub async fn handle_inspect(player_id: i32) -> Result<()> {
    dbg!(make_pidgtm_player_getter_query(player_id).await?);
    Ok(())
}
