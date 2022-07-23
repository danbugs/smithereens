#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::startgg::{Player, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const PIDGTM_PLAYER_GETTER_QUERY: &str = r#"
query PIDGTM_PlayerGetter($playerId: ID!) {
    player(id: $playerId) {
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
    player: Player,
}

#[derive(Serialize)]
pub struct PIDGTM_PlayerGetterVars {
    playerId: i32,
}

impl PIDGTM_PlayerGetterVars {
    pub fn new(playerId: i32) -> Self {
        PIDGTM_PlayerGetterVars { playerId }
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
        .map_err(|_| anyhow::anyhow!("no player under id: '{}'", player_id))?
        .ok_or_else(|| anyhow::anyhow!("no player found for specified playerId: '{}'", player_id))
}
