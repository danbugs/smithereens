#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::{GQLData, GQLVars, Player, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const PIDGTM_PLAYER_GETTER_QUERY: &str = r#"
query PIDGTM_PlayerGetter($playerId: ID!) {
    player(id: $playerId) {
        id
        prefix
        gamerTag
        user {
            slug
        }
    }
}
"#;

#[derive(Debug, Deserialize)]
pub struct PIDGTM_PlayerGetterData {
    pub player: Option<Player>,
}

impl GQLData for PIDGTM_PlayerGetterData {}

impl PIDGTM_PlayerGetterData {
    pub fn empty() -> Self {
        Self { player: None }
    }
}

#[derive(Debug, Serialize)]
pub struct PIDGTM_PlayerGetterVars {
    playerId: i32,
}

impl GQLVars for PIDGTM_PlayerGetterVars {
    fn update(&mut self) -> Self {
        todo!()
    }
}

impl PIDGTM_PlayerGetterVars {
    pub fn new(playerId: i32) -> Self {
        Self { playerId }
    }
}

pub async fn make_pidgtm_player_getter_query(player_id: i32) -> Result<PIDGTM_PlayerGetterData> {
    let sgg = StartGG::connect();
    sgg.gql_client()
        .query_with_vars::<PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars>(
            PIDGTM_PLAYER_GETTER_QUERY,
            PIDGTM_PlayerGetterVars::new(player_id),
        )
        .await
        .transpose()
        .expect("an unexpected error occurred (empty data)")
        .map_err(|e| anyhow::anyhow!(e.message().to_string()))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::queries::player_getter::make_pidgtm_player_getter_query;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    #[tokio::test]
    async fn player_getter() -> Result<()> {
        dbg!(make_pidgtm_player_getter_query(DANTOTTO_PLAYER_ID).await?);
        Ok(())
    }
}
