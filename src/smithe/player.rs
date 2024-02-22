use std::sync::{Arc, Mutex};

use anyhow::Result;

use smithe_database::db_models::player::Player;

use smithe_lib::{
    common::start_read_all_by_increment_execute_finish_maybe_cancel,
    player::{execute, get_all_like, get_player, get_player_from_slug},
    set::{
        get_all_from_player_id, get_competitor_type, get_last_completed_at, get_set_losses_by_dq,
        get_set_losses_without_dqs, get_set_wins_by_dq, get_set_wins_without_dqs, get_winrate,
    },
};
use startgg::queries::set_getter::{make_set_getter_query, SetGetterVars};

use dialoguer::{theme::ColorfulTheme, Select};

pub async fn handle_search(selected_player: &Player) -> Result<()> {
    tracing::info!("🤔 checking if player is cached...");
    let cache = get_all_from_player_id(selected_player.player_id).await?;
    let updated_after = get_last_completed_at(cache);

    let usgv = SetGetterVars::unpaginated_new(
        selected_player.player_id,
        updated_after,
        &selected_player.gamer_tag,
    );

    start_read_all_by_increment_execute_finish_maybe_cancel(
        true,
        Arc::new(Mutex::new(usgv)),
        make_set_getter_query,
        1,
        None,
        execute,
        |curr_page| async move { Ok(curr_page + 1) },
        finish,
        |_curr_page| Ok(()),
    )
    .await
}

pub async fn handle_slug(slug: &str) -> Result<()> {
    tracing::info!("🔍 looking for player with slug provided...");
    let selected_player: Player = get_player_from_slug(slug).await?;
    handle_search(&selected_player).await?;
    
    Ok(())
}

pub async fn handle_id(id: &i32) -> Result<()> {
    tracing::info!("🔍 looking for player with id provided...");
    let selected_player: Player = get_player(*id).await?;
    handle_search(&selected_player).await?;

    Ok(())
}

pub async fn handle_player(tag: &str) -> Result<()> {
    tracing::info!("🔍 looking for players with tags similar to the provided one...");
    let mut matching_players: Vec<Player> = get_all_like(tag).await?;
    matching_players.sort_by_key(|e| e.player_id);

    // cli display
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("❗ These players matched your search:")
        .default(0)
        .items(&matching_players[..])
        .interact()?;
    
    let selected_player = &matching_players[selection];
    handle_search(selected_player).await?;

    Ok(())
}

async fn finish(usgv: Arc<Mutex<SetGetterVars>>) -> Result<()> {
    let pid = usgv.lock().unwrap().playerId;
    println!(
        "🏆 set wins without DQs: {}",
        get_set_wins_without_dqs(pid).await?
    );
    println!(
        "😭 set losses without DQs: {}",
        get_set_losses_without_dqs(pid).await?
    );
    println!("😎 set wins by DQs: {}", get_set_wins_by_dq(pid).await?);
    println!("🤷 set losses by DQs: {}", get_set_losses_by_dq(pid).await?);
    println!("🥇 win-rate: {}%", get_winrate(pid).await?);

    let competitor_type = get_competitor_type(pid).await?;
    println!(
        "🌱 competitor type: {}-{}er",
        competitor_type.0, competitor_type.1
    );

    Ok(())
}
