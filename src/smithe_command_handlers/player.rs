#![allow(unused)]
use std::{collections::HashMap, os, rc, thread, time::Duration};

use anyhow::Result;

use crate::{
    db,
    db_models::{game::Game, player::Player, set::Set, tournament::Tournament},
    queries::set_getter::make_set_getter_query,
    schema::{
        player_games::dsl::*, player_sets::dsl::*, player_tournaments::dsl::*, players::dsl::*,
    },
};

use dialoguer::{theme::ColorfulTheme, Select};
use diesel::{
    dsl::{count, count_star, sql, sum},
    insert_into,
    prelude::*,
    sql_query,
    sql_types::{Float, Int4, Text},
};

pub async fn handle_player(tag: &str) -> Result<()> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    tracing::info!("🔍 looking for players with tags similar to the provided one...");
    let db_connection = db::connect()?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag_with_prefix.ilike(format!("%{}%", processed_tag))) // case-insensitive like
        .get_results::<Player>(&db_connection)?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("❗ These players matched your search:")
        .default(0)
        .items(&matching_players[..])
        .interact()?;

    let selected_player = &matching_players[selection];

    tracing::info!("🤔 checking if player is cached...");
    let cache = player_sets
        .filter(crate::schema::player_sets::requester_id.eq(selected_player.player_id))
        .load::<Set>(&db_connection)?;
    // ^^^ have to use fully-qualified syntax in the filter here

    let updated_after = if !cache.is_empty() {
        tracing::info!("✅ player was cached...");
        Some(
            cache
                .iter()
                .max_by_key(|s| s.completed_at)
                .unwrap()
                .completed_at
                + 1,
        )
    } else {
        tracing::info!("❌ player was not cached...");
        None
    };

    let mut curr_page = 1;
    let mut curated_sets = vec![];
    let mut curated_games = vec![];
    let mut curated_tournaments = vec![];
    loop {
        let mut player = None;

        // there is the possibility we will hit the rate-limit so we want to do this on loop
        loop {
            tracing::info!("🍥 querying StartGG API for player results...");
            match make_set_getter_query(
                selected_player.player_id,
                curr_page,
                updated_after,
                if selected_player.gamer_tag_with_prefix.contains(" | ") {
                    &selected_player.gamer_tag_with_prefix
                        [selected_player.gamer_tag_with_prefix.find(" | ").unwrap() + 3..]
                } else {
                    &selected_player.gamer_tag_with_prefix
                },
            )
            .await
            {
                Ok(sgd) => {
                    player = Some(sgd.player);
                    break;
                }
                Err(e) => {
                    tracing::error!("🐌 hit a snag, backing off: '{:?}'", e);
                    thread::sleep(Duration::from_secs(60));
                }
            }
        }

        let ss = player.unwrap().sets.unwrap().nodes;
        // ^^^ guaranteed to have sets in this context, ok to unwrap

        if ss.is_empty() {
            tracing::info!("🏁 finished compiling results for this player!");
            break;
        } else {
            tracing::info!("✅ got some results...");
            for s in ss {
                // we only want to compile results for: double elimination single ssbu brackets
                if s.event.videogame.as_ref().unwrap().name == "Super Smash Bros. Ultimate"
                    && s.phaseGroup.bracketType == "DOUBLE_ELIMINATION"
                    && s.event.teamRosterSize.is_none()
                {
                    let requester_entrant_id = if s.event.standings.is_some()
                        && !s.event.standings.as_ref().unwrap().nodes.is_empty()
                    {
                        s.event
                            .standings
                            .as_ref()
                            .unwrap()
                            .nodes
                            .iter()
                            .find(|i| i.player.as_ref().unwrap().id.eq(&selected_player.player_id))
                            .unwrap()
                            .entrant
                            .as_ref()
                            .unwrap()
                            .id
                            .as_ref()
                            .unwrap()
                    } else {
                        // this means the standings aren't finished, so the tourney is on going
                        continue;
                    };

                    let gids = s.games.map(|gs| {
                        gs.iter()
                            .map(|g| {
                                let rcp_num = if let Some(rs) = &g.selections {
                                    rs.iter()
                                        .find(|i| {
                                            i.entrant.id.as_ref().unwrap().eq(requester_entrant_id)
                                        })
                                        .map(|rgs| rgs.selectionValue)
                                } else {
                                    None
                                };

                                let ocp_num = if let Some(os) = &g.selections {
                                    os.iter()
                                        .find(|i| {
                                            i.entrant.id.as_ref().unwrap().ne(requester_entrant_id)
                                        })
                                        .map(|ogs| ogs.selectionValue)
                                } else {
                                    None
                                };

                                curated_games.push(Game::new(
                                    g.id,
                                    selected_player.player_id,
                                    g.winnerId.eq(requester_entrant_id),
                                    g.orderNum,
                                    rcp_num,
                                    ocp_num,
                                    g.stage.as_ref().map(|se| se.name.clone()),
                                ));

                                g.id
                            })
                            .collect::<Vec<i32>>()
                    });

                    let rslot = s
                        .slots
                        .iter()
                        .find(|i| i.entrant.id.as_ref().unwrap().eq(requester_entrant_id))
                        .unwrap();
                    let oslot = s
                        .slots
                        .iter()
                        .find(|i| i.entrant.id.as_ref().unwrap().ne(requester_entrant_id))
                        .unwrap();

                    curated_sets.push(Set::new(
                        s.id,
                        s.completedAt,
                        selected_player.player_id,
                        s.event.isOnline.unwrap(),
                        s.event.id.unwrap(),
                        s.event.tournament.as_ref().unwrap().id,
                        gids,
                        rslot.entrant.name.as_ref().unwrap(),
                        rslot.standing.stats.as_ref().unwrap().score.value,
                        rslot.seed.seedNum,
                        oslot.entrant.name.as_ref().unwrap(),
                        oslot.standing.stats.as_ref().unwrap().score.value,
                        oslot.seed.seedNum,
                    ));

                    let pt = s
                        .event
                        .standings
                        .as_ref()
                        .unwrap()
                        .nodes
                        .iter()
                        .find(|i| i.player.as_ref().unwrap().id.eq(&selected_player.player_id))
                        .unwrap()
                        .placement
                        .unwrap();

                    let sd = s
                        .slots
                        .iter()
                        .find(|i| i.entrant.id.as_ref().unwrap().eq(requester_entrant_id))
                        .unwrap()
                        .seed
                        .seedNum;

                    let res_pt = player_tournaments
                        .find((
                            s.event.tournament.as_ref().unwrap().id,
                            s.event.id.unwrap(),
                            selected_player.player_id,
                        ))
                        .first::<Tournament>(&db_connection);

                    let tournament = Tournament::new(
                        s.event.tournament.as_ref().unwrap().id,
                        s.event.id.unwrap(),
                        s.event.name.as_ref().unwrap(),
                        &s.event.tournament.as_ref().unwrap().name,
                        s.event.tournament.as_ref().unwrap().endAt,
                        selected_player.player_id,
                        pt,
                        s.event.numEntrants.unwrap(),
                        sd,
                        format!("https://www.start.gg/{}", s.event.slug.as_ref().unwrap()).as_str(),
                    );

                    if res_pt.is_err() && !curated_tournaments.contains(&tournament) {
                        // ^^^ not found
                        curated_tournaments.push(tournament);
                    }
                }
                // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
                // we want to panic regardless
            }
            curr_page += 1;
        }
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

    let set_wins_without_dqs = player_sets
        .filter(result_type.eq(2))
        .count()
        .get_result::<i64>(&db_connection)?;
    println!("🏆 set wins without DQs: {}", set_wins_without_dqs);

    let set_losses_without_dqs = player_sets
        .filter(result_type.eq(-2))
        .count()
        .get_result::<i64>(&db_connection)?;
    println!("😭 set losses without DQs: {}", set_losses_without_dqs);

    let set_wins_by_dq = player_sets
        .filter(result_type.eq(1))
        .count()
        .get_result::<i64>(&db_connection)?;
    println!("😎 set wins by DQs: {}", set_wins_by_dq);

    let set_losses_by_dq = player_sets
        .filter(result_type.eq(-1))
        .count()
        .get_result::<i64>(&db_connection)?;
    println!("🤷 set losses by DQs: {}", set_losses_by_dq);

    let winrate = ((set_wins_without_dqs as f32)
        / ((set_wins_without_dqs + set_losses_without_dqs) as f32))
        .abs()
        * 100.0;
    println!("🥇 win-rate: {}%", winrate.round());

    let raw_player_results = player_sets
        .filter(crate::schema::player_sets::requester_id.eq(selected_player.player_id))
        .group_by(crate::schema::player_sets::event_id)
        .select((
            crate::schema::player_sets::event_id,
            sql("COUNT(result_type>0 OR NULL)"),
            sql("COUNT(result_type<0 OR NULL)"),
        ))
        .get_results::<(i32, String, String)>(&db_connection)?;
    // ^^^ not sure why but have to get the count as text

    let player_results = raw_player_results
        .iter()
        .map(|i| {
            (
                i.0,
                i.1.chars().nth_back(0).unwrap() as u32,
                i.2.chars().nth_back(0).unwrap() as u32,
            )
        })
        .collect::<Vec<(i32, u32, u32)>>();

    let competitor_type = (
        ((player_results.iter().map(|i| i.1).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
        ((player_results.iter().map(|i| i.2).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
    );
    println!(
        "🌱 competitor type: {}-{}er",
        competitor_type.0,
        competitor_type.1
    );

    Ok(())
}
