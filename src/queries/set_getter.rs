#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::startgg::{Player, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SET_GETTER_QUERY: &str = r#"
query SetGetter($playerId: ID!, $page: Int!) {
    player(id: $playerId) {
        id
    	prefix
        gamerTag
    	sets(page: $page, perPage: 150) {
      	    nodes {
                id
                displayScore
                completedAt
                phaseGroup {
                    bracketType
                }
                event {
                    name
                    isOnline
                    videogame {
                        name
                    }
                    tournament {
                        name
                    }
                }
            }
        }
    }
}
"#;
// ^^^ 
// - 150 per page is about the max we can do
// - wanna filter videogame for Super Smash Bros. Ultimate 
// - wanna filter bracket type for DOUBLE_ELIMINATION

#[derive(Debug, Deserialize)]
pub struct SetGetterData {
    pub player: Option<Player>,
}

impl SetGetterData {
    pub fn empty() -> Self {
        Self { player: None }
    }
}

#[derive(Serialize)]
pub struct SetGetterVars {
    playerId: i32,
    page: i32,
}

impl SetGetterVars {
    pub fn new(playerId: i32, page: i32) -> Self {
        Self { playerId, page }
    }
}

pub async fn make_set_getter_query(player_id: i32, page: i32) -> Result<SetGetterData> {
    let sgg = StartGG::connect();
    sgg.gql_client()
        .query_with_vars::<SetGetterData, SetGetterVars>(
            SET_GETTER_QUERY,
            SetGetterVars::new(player_id, page),
        )
        .await
        .transpose()
        .expect("an unexpected error occurred (empty data)")
        .map_err(|e| anyhow::anyhow!(e.message().to_string()))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::queries::set_getter::make_set_getter_query;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    #[tokio::test]
    async fn set_getter() -> Result<()> {
        dbg!(make_set_getter_query(DANTOTTO_PLAYER_ID, 1).await?);
        Ok(())
    }
}
