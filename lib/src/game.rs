use startgg::Set as SGGSet;
use anyhow::Result;

use diesel::prelude::*;
use smithe_database::{db_models::game::Game, schema::player_games::dsl::*};

pub fn maybe_get_games_from_set(
    player_id: i32,
    requester_entrant_id: i32,
    s: &SGGSet,
    sid: i32,
) -> Option<Vec<Game>> {
    s.clone().games.map(|gs| {
        gs.iter()
            .map(|g| {
                let rcp_num = if let Some(rs) = &g.selections {
                    rs.iter()
                        .find(|i| {
                            if let Some(e) = &i.entrant {
                                e.id.as_ref().unwrap().eq(&requester_entrant_id)
                            } else {
                                false
                            }
                        })
                        .map(|rgs| rgs.selectionValue)
                } else {
                    None
                };

                let ocp_num = if let Some(os) = &g.selections {
                    os.iter()
                        .find(|i| {
                            if let Some(e) = &i.entrant {
                                e.id.as_ref().unwrap().ne(&requester_entrant_id)
                            } else {
                                false
                            }
                        })
                        .map(|ogs| ogs.selectionValue)
                } else {
                    None
                };

                Game::new(
                    g.id,
                    player_id,
                    g.winnerId.as_ref().map(|w| w.eq(&requester_entrant_id)),
                    g.orderNum,
                    rcp_num,
                    ocp_num,
                    g.stage.as_ref().map(|se| se.name.clone()),
                    sid,
                )
            })
            .collect::<Vec<Game>>()
    })
}

// delete a player's games given a requester id
pub fn delete_games_from_requester_id(player_id: i32) -> Result<()> {
    let mut db_connection = smithe_database::connect()?;
    diesel::delete(player_games.filter(requester_id.eq(player_id))).execute(&mut db_connection)?;

    Ok(())
}