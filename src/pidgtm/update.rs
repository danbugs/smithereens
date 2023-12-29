use std::sync::{Arc, Mutex};

use anyhow::Result;

use as_any::Downcast;
use smithe_lib::{
    common::start_read_all_by_increment_execute_finish_maybe_cancel,
    player::{
        get_empty_user_with_slug, get_subsequent_player_id_with_circle_back,
        update_player_in_pidgtm_db,
    },
};
use startgg::{
    queries::player_getter::{
        make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars,
    },
    GQLData,
};

pub async fn handle_update(
    start_at_player_id: Option<i32>,
    end_at_player_id: Option<i32>,
) -> Result<()> {
    // set end_at_player_id to None if it is less than or equal start
    let end_at_player_id = if end_at_player_id.is_some()
        && end_at_player_id.unwrap() <= start_at_player_id.unwrap_or(1000)
    {
        None
    } else {
        end_at_player_id
    };

    start_read_all_by_increment_execute_finish_maybe_cancel(
        true,
        Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())),
        make_pidgtm_player_getter_query,
        start_at_player_id.unwrap_or(1000),
        // ^^^ considering I know that the lowest player_id is 1000, no point in getting it every time
        end_at_player_id,
        execute,
        get_subsequent_player_id_with_circle_back,
        |_gqlv| Ok(()),
        |_curr_page| Ok(()),
    )
    .await?;

    Ok(())
}

fn execute<T>(_: i32, player_getter_data: T) -> Result<bool>
where
    T: GQLData,
{
    let pgd = player_getter_data.downcast_ref::<PIDGTM_PlayerGetterData>();
    let mut player = pgd.as_ref().unwrap().player.clone();
    // vvv have to do this because people might have deleted their account:
    if let Some(pti) = player.as_mut() {
        if pti.user.is_none() {
            tracing::info!("❎ caught a deleted account #1 (id: '{}')...", pti.id);
            pti.user = get_empty_user_with_slug(pti.id)?;
        } else if pti.user.as_ref().unwrap().slug.is_none() {
            tracing::info!("❎ caught a deleted account #2 (id: '{}')...", pti.id);
            pti.user = pti.user.as_mut().map(|u| {
                u.slug = get_empty_user_with_slug(pti.id).unwrap().unwrap().slug;
                // ^^^ didn't want to unwrap here, but I guess it's fine to panic
                u.clone()
            });
        } else {
            tracing::info!("💫 updating player (id: '{}')...", pti.id);
        }

        update_player_in_pidgtm_db(pti)?;
    }

    Ok(false)
}
