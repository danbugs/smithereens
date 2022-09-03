#![allow(unused)]
use std::{thread, time::Duration};

use anyhow::Result;

use crate::{
    db,
    db_models::{player::Player, set::Set},
    queries::set_getter::make_set_getter_query,
    schema::{player_sets::dsl::*, players::dsl::*},
};

use dialoguer::{theme::ColorfulTheme, Select};
use diesel::{insert_into, prelude::*};

pub async fn handle_player(tag: &str) -> Result<()> {
    let processed_tag = tag.replace(" ", "%");
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
        .filter(requester_id.eq(selected_player.player_id))
        .load::<Set>(&db_connection)?;

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
    loop {
        let mut player = None;
        loop {
            match make_set_getter_query(
                selected_player.player_id,
                curr_page,
                updated_after,
                &selected_player.gamer_tag_with_prefix,
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
            let mut curated_sets = vec![];
            // TODO: make curated_games (pre-req games db model)
            // TODO: make curated_tournaments (pre-req tournaments db model)
            for s in ss {
                if s.event.videogame.as_ref().unwrap().name == "Super Smash Bros. Ultimate"
                    && s.phaseGroup.bracketType == "DOUBLE_ELIMINATION"
                {
                    let gids = if let Some(gs) = s.games {
                        Some(gs.iter().map(|g| g.id).collect::<Vec<i32>>())
                    } else {
                        None
                    };

                    // TODO: pass new fields to get requester_tag_with_prefix, requester_score, opponent_tag_with_prefix, opponent_score, and result_type
                    curated_sets.push(Set::new(
                        s.id,
                        s.completedAt,
                        selected_player.player_id,
                        s.event.isOnline.unwrap(),
                        s.event.id.unwrap(),
                        s.event.tournament.unwrap().id,
                        gids,
                    ));
                }
                // ^^^ unwrapping in these instances is fine due to the query context that we are in, if an error occurs,
                // we want to panic regardless
            }

            // TODO: insert into tournaments

            // insert_into(sets)
            //     .values(curated_sets)
            //     .execute(&db_connection)?;

            // TODO: insert into games

            curr_page += 1;
        }
    }
    //     // do a for-loop going through pagination of set data
    //     // each query will look somewhat like this:
    //     // query PlayerGetter($playerId: ID!) {
    //     // 	    player(id: $playerId) {
    //     // 		    prefix
    //     // 		    sets(page: 1, perPage: 150) { # 150 is about the max we can do
    //     //   	    nodes {
    //     //              displayScore
    //     //              # ^^^ needs to be parsed into fields:
    //     //                  - 'against_as',
    //     //                  - 'resultType'
    //     //                      > -2: loss
    //     //                      > -1: loss by DQ
    //     //                      > +1: win by DQ
    //     //                      > +2: win
    //     //                  - 'games_won', and
    //     //                  - 'games_lost'.
    //     //              completedAt
    //     //              phaseGroup {
    //     //                  bracketType # we want DOUBLE_ELIMINATION
    //     //                  }
    //     //              event { # aggregate event and tournament name
    //     //                  name
    //     //                  isOnline
    //     //                  videogame {
    //     //                       name # we want Super Smash Bros. Ultimate
    //     //                       }
    //     //                  tournament {
    //     //                      name
    //     //                      }
    //     //                  }
    //     //              }
    //     //          }
    //     //      }
    //     // }
    //     // if videogame name == Super Smash Bros. Ultimate && bracketType == DOUBLE_ELIMINATION
    //     //      - we add them to an array of sets.

    //     // at this point, it would be useful to add this data onto a player_cache db, so that next time
    //     // we can filter sets updatedAfter: completedAt + 1
    //     // this will make our tool: (1) faster, (2) will require less calls to the api (more efficient),
    //     // and (3) will provide good insight of who's using the tool.

    //     // at this point, we want to analyze the data to get:
    //     //      - total # of set wins (sum(resultType == 2) || sum(resultType == 1)),
    //     //      - total # of losses (sum(resultType == -2) || sum(resultType == -1)),
    //     //      - total # of wins by DQs (sum(resultType == 1)),
    //     //      - total # of losses by DQs, (sum(resultType == -1))
    //     //      - winrate abs((sum(resultType == 2) - sum(resultType == -2)) / (sum(resultType == 2) + sum(resultType == -2))),
    //     //      - placements
    //     //      ^^^ (display event + tournament name -> sets // seeding in tournament // result // # wins - # losses), and
    //     //      - what competitor type you are (e.g., 0-2er, 1-2er, 2-2er, etc.).
    //     //      ^^^ sum(resultType == 2) in event + tournament name and sum(resultType == -2) in event + tournament name

    todo!("player functionality still need to be implemented");
}
