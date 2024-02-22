use anyhow::Result;
use diesel::{
    dsl::sql,
    prelude::*,
    sql_types::{BigInt, Integer, VarChar},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use serde::Serialize;
use smithe_database::{db_models::set::Set, schema::player_sets::dsl::*};

use startgg::{Set as SGGSet, SetSlot as SGGSetSlot};

#[derive(Debug, Serialize, QueryableByName)]
pub struct HeadToHeadResult {
    #[diesel(sql_type = VarChar)]
    pub opponent_tag: String,
    #[diesel(sql_type = BigInt)]
    pub total_sets: i64,
    #[diesel(sql_type = BigInt)]
    pub wins: i64,
    #[diesel(sql_type = BigInt)]
    pub losses: i64,
}

pub async fn get_head_to_head_record(requester_id_param: i32) -> Result<Vec<HeadToHeadResult>> {
    let mut db_connection = smithe_database::connect().await?;

    let results = diesel::sql_query(
        "SELECT opponent_tag_with_prefix AS opponent_tag, COUNT(*) AS total_sets, 
        SUM(CASE WHEN requester_score > opponent_score THEN 1 ELSE 0 END) AS wins, 
        SUM(CASE WHEN requester_score < opponent_score THEN 1 ELSE 0 END) AS losses 
        FROM player_sets 
        WHERE requester_id = $1 
        GROUP BY opponent_tag_with_prefix 
        ORDER BY random()",
    )
    .bind::<Integer, _>(requester_id_param)
    .load::<HeadToHeadResult>(&mut db_connection)
    .await?;

    Ok(results)
}

pub async fn get_all_from_player_id(player_id: i32) -> Result<Vec<Set>> {
    let mut db_connection = smithe_database::connect().await?;
    get_all_from_player_id_provided_connection(player_id, &mut db_connection).await
}

async fn get_all_from_player_id_provided_connection(
    player_id: i32,
    db_connection: &mut AsyncPgConnection,
) -> Result<Vec<Set>> {
    let cache = player_sets
        .filter(requester_id.eq(player_id))
        .load::<Set>(db_connection)
        .await?;

    Ok(cache)
}

pub fn get_last_completed_at(cache: Vec<Set>) -> Option<i64> {
    if !cache.is_empty() {
        let last_completed_at = cache
            .iter()
            .max_by_key(|s| s.completed_at)
            .unwrap()
            .completed_at
            + 2;

        tracing::info!(
            "✅ player was cached, last completed_at: {}",
            last_completed_at
        );

        Some(last_completed_at)
    } else {
        tracing::info!("❌ player was not cached...");
        None
    }
}

/// Provides a set with access to:
/// - entrant name,
/// - entrant seed, and
/// - entrant set score (e.g., won 2 games, DQd, etc.).
pub fn get_requester_set_slot(requester_entrant_id: i32, s: &SGGSet) -> Option<SGGSetSlot> {
    s.slots
        .iter()
        .find(|i| {
            if let Some(e) = i.entrant.as_ref() {
                e.id.as_ref().unwrap().eq(&requester_entrant_id)
            } else {
                false
            }
        })
        .cloned()
}

/// Provides a set with access to:
/// - entrant name,
/// - entrant seed, and
/// - entrant set score (e.g., won 2 games, DQd, etc.).
pub fn get_opponent_set_slot(requester_entrant_id: i32, s: &SGGSet) -> Option<SGGSetSlot> {
    s.slots
        .iter()
        .find(|i| {
            if let Some(e) = i.entrant.as_ref() {
                e.id.as_ref().unwrap().ne(&requester_entrant_id)
            } else {
                false
            }
        })
        .cloned()
}

pub async fn get_set_wins_without_dqs(player_id: i32) -> Result<i64> {
    let mut db_connection = smithe_database::connect().await?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(2))
        .count()
        .get_result::<i64>(&mut db_connection)
        .await?)
}

// delete a player's sets given a requester_id
pub async fn delete_sets_by_requester_id(player_id: i32) -> Result<()> {
    let mut db_connection = smithe_database::connect().await?;
    delete_sets_by_requester_id_provided_connection(player_id, &mut db_connection).await?;
    Ok(())
}

async fn delete_sets_by_requester_id_provided_connection(
    player_id: i32,
    db_connection: &mut AsyncPgConnection,
) -> Result<()> {
    diesel::delete(player_sets.filter(requester_id.eq(player_id)))
        .execute(db_connection)
        .await?;
    Ok(())
}

pub async fn get_set_losses_without_dqs(player_id: i32) -> Result<i64> {
    let mut db_connection = smithe_database::connect().await?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(-2))
        .count()
        .get_result::<i64>(&mut db_connection)
        .await?)
}

pub async fn get_set_wins_by_dq(player_id: i32) -> Result<i64> {
    let mut db_connection = smithe_database::connect().await?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(1))
        .count()
        .get_result::<i64>(&mut db_connection)
        .await?)
}

pub async fn get_set_losses_by_dq(player_id: i32) -> Result<i64> {
    let mut db_connection = smithe_database::connect().await?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .filter(result_type.eq(-1))
        .count()
        .get_result::<i64>(&mut db_connection)
        .await?)
}

pub async fn get_winrate(player_id: i32) -> Result<f32> {
    let set_wins_without_dqs = get_set_wins_without_dqs(player_id).await?;
    let set_losses_without_dqs = get_set_losses_without_dqs(player_id).await?;
    Ok(
        ((set_wins_without_dqs as f32) / ((set_wins_without_dqs + set_losses_without_dqs) as f32))
            .abs()
            * 100.0,
    )
}

// get sets per player id
pub async fn get_sets_per_player_id(player_id: i32) -> Result<Vec<Set>> {
    let mut db_connection = smithe_database::connect().await?;
    Ok(player_sets
        .filter(smithe_database::schema::player_sets::requester_id.eq(player_id))
        .get_results::<Set>(&mut db_connection)
        .await?)
}

pub async fn get_competitor_type(player_id: i32) -> Result<(u32, u32)> {
    let mut db_connection = smithe_database::connect().await?;
    let raw_player_results = player_sets
        .filter(requester_id.eq(player_id))
        .group_by(event_id)
        .select((
            event_id,
            sql::<BigInt>("COUNT(result_type>1 OR NULL)"),
            sql::<BigInt>("COUNT(result_type<-1 OR NULL)"),
        ))
        .get_results::<(i32, i64, i64)>(&mut db_connection)
        .await?;

    let player_results = raw_player_results
        .iter()
        .map(|(eid, win_count, loss_count)| {
            let win_count = *win_count as u32; // Assuming win_count is already i64 and within u32 range
            let loss_count = *loss_count as u32; // Assuming loss_count is already i64 and within u32 range
            (*eid, win_count, loss_count)
        })
        .collect::<Vec<(i32, u32, u32)>>();

    // filter out events where both player_results.1 and player_results.2 are 0
    let player_results = player_results
        .iter()
        .filter(|i| i.1 != 0 || i.2 != 0)
        .collect::<Vec<&(i32, u32, u32)>>();
    Ok((
        ((player_results.iter().map(|i| i.1).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
        ((player_results.iter().map(|i| i.2).sum::<u32>() as f32) / (player_results.len() as f32))
            .round() as u32,
    ))
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::common::get_sggset_test_data;

    use super::*;
    use diesel_async::scoped_futures::ScopedFutureExt;
    use diesel_async::AsyncConnection;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_head_to_head_record() -> Result<()> {
        get_head_to_head_record(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_all_from_player_id() -> Result<()> {
        get_all_from_player_id(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_last_completed_at() -> Result<()> {
        let cache = get_all_from_player_id(DANTOTTO_PLAYER_ID).await?;
        get_last_completed_at(cache).expect("failed to get last completed_at");
        Ok(())
    }

    #[test]
    fn test_get_requester_set_slot() -> Result<()> {
        let set = get_sggset_test_data();
        get_requester_set_slot(9410060, &set).expect("failed to get requester set slot");
        Ok(())
    }

    #[test]
    fn test_get_opponent_set_slot() -> Result<()> {
        let set = get_sggset_test_data();
        get_opponent_set_slot(9412484, &set).expect("failed to get opponent set slot");
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_set_wins_without_dqs() -> Result<()> {
        get_set_wins_without_dqs(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_delete_sets_by_requester_id() -> Result<()> {
        let mut db_connection = smithe_database::connect().await?;
        let err = db_connection
            .transaction::<(), _, _>(|db_connection| {
                async {
                    delete_sets_by_requester_id_provided_connection(
                        DANTOTTO_PLAYER_ID,
                        db_connection,
                    )
                    .await
                    .expect("failed to delete sets by requester id");

                    // check that there are no sets for the player
                    let sets = get_all_from_player_id_provided_connection(
                        DANTOTTO_PLAYER_ID,
                        db_connection,
                    )
                    .await
                    .expect("failed to get sets");
                    assert!(sets.is_empty());

                    Err(diesel::result::Error::RollbackTransaction)
                }
                .scope_boxed()
            })
            .await;

        assert!(err.is_err());

        // check that there are sets for the player
        let sets = get_all_from_player_id(DANTOTTO_PLAYER_ID).await?;
        assert!(!sets.is_empty());

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_set_losses_without_dqs() -> Result<()> {
        get_set_losses_without_dqs(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_set_wins_by_dq() -> Result<()> {
        get_set_wins_by_dq(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_set_losses_by_dq() -> Result<()> {
        get_set_losses_by_dq(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_winrate() -> Result<()> {
        get_winrate(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_sets_per_player_id() -> Result<()> {
        get_sets_per_player_id(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_get_competitor_type() -> Result<()> {
        get_competitor_type(DANTOTTO_PLAYER_ID).await?;
        Ok(())
    }
}
