use anyhow::Result;

use as_any::Downcast;
use diesel::{insert_into, prelude::*};
use smithe_database::db_models::empty_player_ids::EmptyPlayerId;

use smithe_database::db_models::player::Player;
use smithe_database::schema::empty_player_ids::dsl::*;

use smithe_database::schema::players::dsl::*;
use smithe_lib::common::start_read_all_execute_finish_maybe_cancel;
use smithe_lib::player::{get_last_cached_player_id, increment_last_cached_player_id};
use startgg::queries::player_getter::{
    make_pidgtm_player_getter_query, PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars,
};
use startgg::GQLData;



use std::sync::{Arc, Mutex};



pub async fn handle_map() -> Result<()> {
    start_read_all_execute_finish_maybe_cancel(
        Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())),
        make_pidgtm_player_getter_query,
        get_last_cached_player_id,
        execute,
        |_gqlv| Ok(()),
        increment_last_cached_player_id,
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
    let db_connection = smithe_database::connect()?;
    let curr_player_id = player_getter_vars.lock().unwrap().playerId;
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
            insert_into(players)
                .values(Player::from(pti.clone()))
                .execute(&db_connection)?;
        }
    } else {
        tracing::info!("⛔ no player under id '{}', moving on...", curr_player_id);
        insert_into(empty_player_ids)
            .values(EmptyPlayerId::from(curr_player_id))
            .execute(&db_connection)?;
    }

    Ok(false)
}
