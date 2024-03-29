use anyhow::Result;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use startgg::Set as SGGSet;

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
                        .and_then(|rgs| rgs.selectionValue)
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
                        .and_then(|ogs| ogs.selectionValue)
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
pub async fn delete_games_from_requester_id(player_id: i32) -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;
    delete_games_from_requester_id_provided_connection(player_id, &mut db_connection).await?;

    Ok(())
}

// delete a player's games given a requester id
async fn delete_games_from_requester_id_provided_connection(
    player_id: i32,
    db_connection: &mut AsyncPgConnection,
) -> Result<()> {
    diesel::delete(player_games.filter(requester_id.eq(player_id)))
        .execute(db_connection)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use diesel_async::scoped_futures::ScopedFutureExt;
    use diesel_async::AsyncConnection;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    // test delete_games_from_requester_id w/ transactions
    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_delete_games_from_requester_id() -> Result<()> {
        let mut db_connection = smithe_database::connect().await?;

        let err = db_connection
            .transaction::<(), _, _>(|db_connection| {
                async {
                    delete_games_from_requester_id_provided_connection(
                        DANTOTTO_PLAYER_ID,
                        db_connection,
                    )
                    .await
                    .expect("failed to delete games");
                    assert_eq!(
                        player_games
                            .filter(requester_id.eq(DANTOTTO_PLAYER_ID))
                            .count()
                            .get_result::<i64>(db_connection)
                            .await?,
                        0
                    );

                    Err(diesel::result::Error::RollbackTransaction)
                }
                .scope_boxed()
            })
            .await;

        assert!(err.is_err());

        // check that there are still games under the player's id
        assert_ne!(
            player_games
                .filter(requester_id.eq(DANTOTTO_PLAYER_ID))
                .count()
                .get_result::<i64>(&mut db_connection)
                .await?,
            0
        );

        Ok(())
    }
}
