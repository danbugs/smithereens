use anyhow::Result;

use crate::{db, db_models::player::Player, schema::players::dsl::*};
use diesel::prelude::*;

pub fn handle_player(tag: &str) -> Result<()> {
    tracing::info!("querying pidgtm db for players with tag similar to the provided ones...");
    let db_connection = db::connect()?;
    let res = players
        .filter(gamer_tag_with_prefix.ilike(format!("%{}%", tag)))
        .get_results::<Player>(&db_connection)?;
    dbg!(res);
    // query db to get player, get vector of players matching tag
    // ^^^ ILIKE '%PLAYER_TAG%'

    // display dialoguer prompt to select the desired player

    // check if user is in cache, if yes, get 'completedAt' int.

    // do a for-loop going through pagination of set data
    // each query will look somewhat like this:
    // query PlayerGetter($playerId: ID!) {
    // 	    player(id: $playerId) {
    // 		    prefix
    // 	        gamerTag
    // 		    sets(page: 1, perPage: 150) { # 150 is about the max we can do
    //   	    nodes {
    //              displayScore
    //              # ^^^ needs to be parsed into fields:
    //                  - 'against_as',
    //                  - 'resultType'
    //                      > -2: loss
    //                      > -1: loss by DQ
    //                      > +1: win by DQ
    //                      > +2: win
    //                  - 'games_won', and
    //                  - 'games_lost'.
    //              state
    //              completedAt
    //              phaseGroup {
    //                  bracketType # we want DOUBLE_ELIMINATION
    //                  }
    //              event { # aggregate event and tournament name
    //                  name
    //                  isOnline
    //                  videogame {
    //                       name # we want Super Smash Bros. Ultimate
    //                       }
    //                  tournament {
    //                      name
    //                      }
    //                  }
    //              }
    //          }
    //      }
    // }
    // if videogame name == Super Smash Bros. Ultimate && bracketType == DOUBLE_ELIMINATION
    //      - we add them to an array of sets.

    // at this point, it would be useful to add this data onto a player_cache db, so that next time
    // we can filter sets updatedAfter: completedAt + 1
    // this will make our tool: (1) faster, (2) will require less calls to the api (more efficient),
    // and (3) will provide good insight of who's using the tool.

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
