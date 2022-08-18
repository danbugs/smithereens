use anyhow::Result;

use crate::{
    db,
    db_models::{player::Player, set::Set},
    queries::set_getter::make_set_getter_query,
    schema::{players::dsl::*, sets::dsl::*},
};
use dialoguer::{theme::ColorfulTheme, Select};
use diesel::prelude::*;

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
    let cache = sets
        .filter(requester_id.eq(selected_player.player_id))
        .load::<Set>(&db_connection)?;
    dbg!(cache);

    let mut curr_page = 1;
    loop {
        let player = make_set_getter_query(selected_player.player_id, curr_page)
            .await?
            .player
            .unwrap();
        // ^^^ ok to unwrap, due to constrained player selection

        let ss = player.sets.unwrap().nodes;
        // ^^^ guaranteed to have sets in this context, ok to unwrap

        if ss.is_empty() {
            break;
        } else {
            for s in ss {
                // add each set to cache
                // get the following fields:
                // - completed_at: i64,
                // - requester_id: i32,
                // - requester_tag_with_prefix: String,
                // - requester_score: i32,
                // - opponent_tag_with_prefix: String,
                // - opponent_score: i32,
                // - result_type: i32,
                // - event_at_tournament: String,
                // - is_event_online: bool
                if s.event.videogame.name == "Super Smash Bros. Ultimate"
                    && s.phaseGroup.bracketType == "DOUBLE_ELIMINATION"
                {
                    dbg!(s);
                }
            }

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
