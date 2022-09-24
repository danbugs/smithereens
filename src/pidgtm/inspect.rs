use std::sync::{Arc, Mutex};

use anyhow::Result;

use startgg::queries::player_getter::{make_pidgtm_player_getter_query, PIDGTM_PlayerGetterVars};

pub async fn handle_inspect(player_id: i32) -> Result<()> {
    dbg!(
        make_pidgtm_player_getter_query(
            player_id,
            Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty()))
        )
        .await?
    );
    Ok(())
}
