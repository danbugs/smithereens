use std::sync::{Arc, Mutex};

use anyhow::Result;

use as_any::Downcast;
use smithe_lib::{
    common::start_read_all_by_increment_execute_finish_maybe_cancel,
    player::{get_subsequent_player_id_with_circle_back, update_player_in_pidgtm_db},
};
use startgg::{
    queries::player_getter::{
        make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars,
    },
    GQLData,
};

pub async fn handle_update() -> Result<()> {
    start_read_all_by_increment_execute_finish_maybe_cancel(
        Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())),
        make_pidgtm_player_getter_query,
        || Ok(1000),
        // ^^^ considering I know that the lowest player_id is 1000, no point in getting it every time
        execute,
        get_subsequent_player_id_with_circle_back,
        |_gqlv| Ok(()),
        |_gqlv| Ok(()),
    )
    .await?;

    Ok(())
}

fn execute<T>(
    player_getter_vars: Arc<Mutex<PIDGTM_PlayerGetterVars>>,
    player_getter_data: T,
) -> Result<bool>
where
    T: GQLData,
{
    let curr_player_id = player_getter_vars.lock().unwrap().playerId;
    let pgd = player_getter_data.downcast_ref::<PIDGTM_PlayerGetterData>();
    if let Some(pti) = &pgd.as_ref().unwrap().player {
        tracing::info!("💫 updating player (id: '{}')...", curr_player_id);
        update_player_in_pidgtm_db(pti)?;
    }

    Ok(false)
}
