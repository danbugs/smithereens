use std::sync::{Arc, Mutex};

use anyhow::Result;

use as_any::Downcast;
use smithe_database::{
    db_models::{player::Player, set::Set, tournament::Tournament},
    schema::{player_games::dsl::*, player_sets::dsl::*, player_tournaments::dsl::*},
};

use smithe_lib::{
    common::start_read_all_execute_finish_maybe_cancel,
    game::maybe_get_games_from_set,
    player::{get_all_like, maybe_remove_prefix_from_gamer_tag},
    set::{
        get_all_from_player_id, get_competitor_type, get_last_completed_at, get_opponent_set_slot,
        get_requester_set_slot, get_set_losses_by_dq, get_set_losses_without_dqs,
        get_set_wins_by_dq, get_set_wins_without_dqs, get_winrate,
    },
    tournament::{
        get_placement, get_requester_id_from_standings, get_seed,
        is_ssbu_singles_double_elimination_tournament, is_tournament_cached,
        is_tournament_finished,
    },
};
use startgg::{
    queries::set_getter::{make_set_getter_query, SetGetterData, SetGetterVars},
    GQLData,
};

use dialoguer::{theme::ColorfulTheme, Select};
use diesel::{insert_into, prelude::*};

pub async fn handle_player(tag: &str) -> Result<()> {
    tracing::info!("🔍 looking for players with tags similar to the provided one...");
    let matching_players: Vec<Player> = get_all_like(tag)?;

    // cli display
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("❗ These players matched your search:")
        .default(0)
        .items(&matching_players[..])
        .interact()?;
    let selected_player = &matching_players[selection];

    tracing::info!("🤔 checking if player is cached...");
    let cache = get_all_from_player_id(selected_player.player_id)?;
    let updated_after = get_last_completed_at(cache);
    let processed_gamer_tag = maybe_remove_prefix_from_gamer_tag(selected_player);

    let usgv = SetGetterVars::unpaginated_new(
        selected_player.player_id,
        updated_after,
        &processed_gamer_tag,
    );

    start_read_all_execute_finish_maybe_cancel(
        Arc::new(Mutex::new(usgv)),
        make_set_getter_query,
        || Ok(1),
        execute,
        finish,
        |gqlv| Ok(()),
    )
    .await
}

fn execute<T>(_: Arc<Mutex<SetGetterVars>>, set_getter_data: T) -> Result<bool>
where
    T: GQLData,
{
    let sgd = set_getter_data.downcast_ref::<SetGetterData>();
    let player = sgd.unwrap().player.clone();

    let mut curated_sets = vec![];
    let mut curated_games = vec![];
    let mut curated_tournaments = vec![];

    let db_connection = smithe_database::connect()?;

    let ss = player.sets.unwrap().nodes;
    // ^^^ guaranteed to have sets in this context, ok to unwrap

    if ss.is_empty() {
        tracing::info!("🏁 finished compiling results for this player!");
        Ok(true)
    } else {
        tracing::info!("✅ got some results...");
        for s in ss {
            // we only want to compile results for: double elimination single ssbu brackets
            if is_ssbu_singles_double_elimination_tournament(&s) {
                let requester_entrant_id = if is_tournament_finished(&s) {
                    get_requester_id_from_standings(&s, player.id)
                } else {
                    continue;
                };

                let maybe_games = maybe_get_games_from_set(player.id, requester_entrant_id, &s);

                // if there are games, we want to add to the vec to insert in the DB at the end
                if let Some(mut games) = maybe_games.clone() {
                    curated_games.append(&mut games);
                }

                let rslot = get_requester_set_slot(requester_entrant_id, &s);
                let oslot = get_opponent_set_slot(requester_entrant_id, &s);

                curated_sets.push(Set::new(
                    s.id,
                    s.completedAt,
                    player.id,
                    s.event.isOnline.unwrap(),
                    s.event.id.unwrap(),
                    s.event.tournament.as_ref().unwrap().id,
                    maybe_games.clone(),
                    rslot.entrant.name.as_ref().unwrap(),
                    rslot.standing.stats.as_ref().unwrap().score.value,
                    rslot.seed.seedNum,
                    oslot.entrant.name.as_ref().unwrap(),
                    oslot.standing.stats.as_ref().unwrap().score.value,
                    oslot.seed.seedNum,
                ));

                let tournament = Tournament::new(
                    s.event.tournament.as_ref().unwrap().id,
                    s.event.id.unwrap(),
                    s.event.name.as_ref().unwrap(),
                    &s.event.tournament.as_ref().unwrap().name,
                    s.event.tournament.as_ref().unwrap().endAt,
                    player.id,
                    get_placement(player.id, &s),
                    s.event.numEntrants.unwrap(),
                    get_seed(requester_entrant_id, &s),
                    format!("https://www.start.gg/{}", s.event.slug.as_ref().unwrap()).as_str(),
                );

                if !is_tournament_cached(player.id, &s)?
                    && !curated_tournaments.contains(&tournament)
                {
                    // ^^^ not found
                    curated_tournaments.push(tournament);
                }
            }
            // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
            // we want to panic regardless
        }

        insert_into(player_games)
            .values(curated_games)
            .execute(&db_connection)?;

        insert_into(player_sets)
            .values(curated_sets)
            .execute(&db_connection)?;

        insert_into(player_tournaments)
            .values(curated_tournaments)
            .execute(&db_connection)?;

        Ok(false)
    }
}

fn finish(usgv: Arc<Mutex<SetGetterVars>>) -> Result<()> {
    let pid = usgv.lock().unwrap().playerId;
    println!(
        "🏆 set wins without DQs: {}",
        get_set_wins_without_dqs(pid)?
    );
    println!(
        "😭 set losses without DQs: {}",
        get_set_losses_without_dqs(pid)?
    );
    println!("😎 set wins by DQs: {}", get_set_wins_by_dq(pid)?);
    println!("🤷 set losses by DQs: {}", get_set_losses_by_dq(pid)?);
    println!("🥇 win-rate: {}%", get_winrate(pid)?);

    let competitor_type = get_competitor_type(pid)?;
    println!(
        "🌱 competitor type: {}-{}er",
        competitor_type.0, competitor_type.1
    );

    Ok(())
}
