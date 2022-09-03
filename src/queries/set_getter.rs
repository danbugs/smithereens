#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::startgg::{Player, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SET_GETTER_QUERY: &str = r#"
query SetGetter($playerId: ID!, $page: Int!, $updatedAfter: Timestamp, $gamerTag: String) {
	player(id: $playerId) {
		id
		sets(page: $page, perPage: 10, filters: {updatedAfter: $updatedAfter}) {
			nodes {
				id
				games {
   	   		id
					orderNum
          selections {
            selectionValue
          }
          stage {
            name
          }
        }
        slots {
          entrant {
            name
          }
          seed {
            seedNum
          }
          standing {
            stats {
              score {
                value
              }
            }
          }
        }
        completedAt
        phaseGroup {
          bracketType
        }
        event {
          id
          name
          numEntrants
          isOnline
          videogame {
            name
          }
          tournament {
            id
            name
          }
          standings(query: { filter: { search: { searchString: $gamerTag}}}) {
            nodes {
              player {
                id
              }
              placement
            }
          }
        }
      }
    }
  }
}
"#;
// ^^^
// - 40 per page is about the max we can do
// - wanna filter videogame for Super Smash Bros. Ultimate
// - wanna filter bracket type for DOUBLE_ELIMINATION

#[derive(Debug, Deserialize)]
pub struct SetGetterData {
    pub player: Player,
}

#[derive(Serialize)]
pub struct SetGetterVars {
    playerId: i32,
    page: i32,
    updatedAfter: Option<i64>,
    gamerTag: String,
}

impl SetGetterVars {
    pub fn new(playerId: i32, page: i32, updatedAfter: Option<i64>, gamerTag: &str) -> Self {
        Self {
            playerId,
            page,
            updatedAfter,
            gamerTag: gamerTag.to_string(),
        }
    }
}

pub async fn make_set_getter_query(
    player_id: i32,
    page: i32,
    updated_after: Option<i64>,
    gamer_tag: &str,
) -> Result<SetGetterData> {
    let sgg = StartGG::connect();
    sgg.gql_client()
        .query_with_vars::<SetGetterData, SetGetterVars>(
            SET_GETTER_QUERY,
            SetGetterVars::new(player_id, page, updated_after, gamer_tag),
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
        dbg!(make_set_getter_query(DANTOTTO_PLAYER_ID, 1, None, "Dantotto").await?);
        Ok(())
    }
}
