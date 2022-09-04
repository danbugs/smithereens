#![allow(unused)]
use std::{os, rc, thread, time::Duration};

use anyhow::Result;

use crate::{
    db,
    db_models::{game::Game, player::Player, set::Set, tournament::Tournament},
    queries::set_getter::make_set_getter_query,
    schema::{player_sets::dsl::*, player_tournaments::dsl::*, players::dsl::*},
};

use dialoguer::{theme::ColorfulTheme, Select};
use diesel::{insert_into, prelude::*};

pub async fn handle_player(tag: &str) -> Result<()> {
    let processed_tag = tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search most inclusive

    tracing::info!("querying pidgtm db for players with tag similar to the provided ones...");
    let db_connection = db::connect()?;
    let matching_players: Vec<Player> = players
        .filter(gamer_tag_with_prefix.ilike(format!("%{}%", processed_tag)))
        .get_results::<Player>(&db_connection)?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("These players matched your search:")
        .default(0)
        .items(&matching_players[..])
        .interact()?;

    let selected_player = &matching_players[selection];

    tracing::info!("checking if player is cached...");
    let db_connection = db::connect()?;
    let cache = player_sets
        .filter(crate::schema::player_sets::requester_id.eq(selected_player.player_id))
        .load::<Set>(&db_connection)?;
    // ^^^ have to use fully-qualified syntax in the filter here

    let updated_after = if !cache.is_empty() {
        Some(
            cache
                .iter()
                .max_by_key(|s| s.completed_at)
                .unwrap()
                .completed_at,
        )
    } else {
        None
    };

    let mut curr_page = 1;
    let mut curated_sets = vec![];
    let mut curated_games = vec![];
    let mut curated_tournaments = vec![];
    loop {
        let mut player = None;
        loop {
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
            break;
        } else {
            for s in ss {
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

                    let gids = if let Some(gs) = s.games {
                        Some(
                            gs.iter()
                                .map(|g| {
                                    let rcp_num = if let Some(rs) = &g.selections {
                                        if let Some(rgs) = rs.iter().find(|i| {
                                            i.entrant.id.as_ref().unwrap().eq(requester_entrant_id)
                                        }) {
                                            Some(rgs.selectionValue)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    };

                                    let ocp_num = if let Some(os) = &g.selections {
                                        if let Some(ogs) = os.iter().find(|i| {
                                            i.entrant.id.as_ref().unwrap().ne(requester_entrant_id)
                                        }) {
                                            Some(ogs.selectionValue)
                                        } else {
                                            None
                                        }
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
                                        if let Some(se) = &g.stage {
                                            Some(se.name.clone())
                                        } else {
                                            None
                                        },
                                    ));

                                    g.id
                                })
                                .collect::<Vec<i32>>(),
                        )
                    } else {
                        None
                    };

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
                        &rslot.entrant.name.as_ref().unwrap(),
                        rslot.standing.stats.as_ref().unwrap().score.value,
                        rslot.seed.seedNum,
                        &oslot.entrant.name.as_ref().unwrap(),
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
                        selected_player.player_id,
                        pt,
                        s.event.numEntrants.unwrap(),
                        sd,
                        format!("https://www.start.gg/{}", s.event.slug.as_ref().unwrap()).as_str(),
                    );

                    if res_pt.is_err() && !curated_tournaments.contains(&tournament) {
                        // ^^^ not found
                        tracing::info!("{:?}", &tournament);
                        curated_tournaments.push(tournament);
                    }
                }
                // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
                // we want to panic regardless
            }
            curr_page += 1;
        }
    }

    // TODO: insert into tournaments

    // TODO:
    // insert_into(sets)
    //     .values(curated_sets)
    //     .execute(&db_connection)?;

    // TODO: insert into games

    // TODO:
    // at this point, we want to analyze the data to get:
    //      - total # of set wins (sum(resultType == 2) || sum(resultType == 1)),
    //      - total # of losses (sum(resultType == -2) || sum(resultType == -1)),
    //      - total # of wins by DQs (sum(resultType == 1)),
    //      - total # of losses by DQs, (sum(resultType == -1))
    //      - winrate abs((sum(resultType == 2) - sum(resultType == -2)) / (sum(resultType == 2) + sum(resultType == -2))),
    //      - placements
    //      ^^^ (display event + tournament name -> sets // seeding in tournament // result // # wins - # losses), and
    //      - what competitor type you are (e.g., 0-2er, 1-2er, 2-2er, etc.).
    //      ^^^ sum(resultType == 2) in event + tournament name and sum(resultType == -2) in event + tournament name

    todo!("player functionality still need to be implemented");
}
