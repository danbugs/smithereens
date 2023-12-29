use anyhow::Result;

use as_any::Downcast;
use smithe_lib::common::start_read_all_by_increment_execute_finish_maybe_cancel;
use smithe_lib::player::{
    add_new_empty_player_record, add_new_player_to_pidgtm_db,
    check_if_large_consecutive_playerid_grouping_exists,
    delete_large_consecutive_playerid_grouping, get_last_cached_player_id, get_max_player_id,
};
use startgg::queries::player_getter::{
    make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars,
};
use startgg::GQLData;

use std::sync::{Arc, Mutex};

pub async fn handle_map(
    start_at_player_id: Option<i32>,
    end_at_player_id: Option<i32>,
) -> Result<()> {
    let start = start_at_player_id.unwrap_or(get_last_cached_player_id()?);

    // set end_at_player_id to None if it is less than or equal start
    let end_at_player_id = if end_at_player_id.is_some() && end_at_player_id.unwrap() <= start {
        None
    } else {
        end_at_player_id
    };

    start_read_all_by_increment_execute_finish_maybe_cancel(
        true,
        Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())),
        make_pidgtm_player_getter_query,
        start,
        end_at_player_id,
        execute,
        increment,
        |_gqlv| Ok(()),
        |_gqlv| Ok(()),
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

fn increment(curr_player_id: i32) -> Result<i32> {
    // Check if there is a consecutive grouping larger than 1144 players.
    // If so, that means we probably reached the last page of players.
    if check_if_large_consecutive_playerid_grouping_exists()? {
        tracing::info!("🏁 reached the end of the player list!");
        tracing::info!("🗑️ deleting large consecutive player id grouping...");
        tracing::info!("🔁 restarting from largest player id...");
        delete_large_consecutive_playerid_grouping()?;
        get_max_player_id()
    }
    // If not, increment the player id by 1.
    else {
        Ok(curr_player_id + 1)
    }
}
