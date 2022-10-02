use anyhow::Result;

use as_any::Downcast;
use smithe_lib::common::start_read_all_by_increment_execute_finish_maybe_cancel;
use smithe_lib::player::{
    add_new_empty_player_record, add_new_player_to_pidgtm_db, get_last_cached_player_id,
    increment_last_cached_player_id,
};
use startgg::queries::player_getter::{
    make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars,
};
use startgg::GQLData;

use std::sync::{Arc, Mutex};

pub async fn handle_map() -> Result<()> {
    start_read_all_by_increment_execute_finish_maybe_cancel(
        Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())),
        make_pidgtm_player_getter_query,
        get_last_cached_player_id()?,
        execute,
        |curr_page| Ok(curr_page + 1),
        |_gqlv| Ok(()),
        increment_last_cached_player_id,
    )
    .await?;
    Ok(())
}

fn execute<T>(curr_player_id: i32, player_getter_data: T) -> Result<bool>
where
    T: GQLData,
{
    let pgd = player_getter_data.downcast_ref::<PIDGTM_PlayerGetterData>();
    if let Some(pti) = &pgd.as_ref().unwrap().player {
        if pti.user.is_none() || pti.user.as_ref().unwrap().slug.is_none() {
            tracing::info!(
                "🧪 caught a test account (id: '{}'), skipping addition to pidgtm db...",
                curr_player_id
            );
        } else {
            tracing::info!(
                "💫 appending player (id: '{}') to pidgtm db...",
                curr_player_id
            );
            add_new_player_to_pidgtm_db(pti)?;
        }
    } else {
        tracing::info!("⛔ no player under id '{}', moving on...", curr_player_id);
        add_new_empty_player_record(curr_player_id)?;
    }

    Ok(false)
}
