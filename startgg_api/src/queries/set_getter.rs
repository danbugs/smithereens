#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use crate::{GQLData, GQLVars, Player, StartGG};
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
          winnerId
					orderNum
          selections {
            entrant {
              id
              name
            }
            selectionValue
          }
          stage {
            name
          }
        }
        slots {
          entrant {
            id
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
          slug
          name
          numEntrants
          isOnline
          teamRosterSize {
            maxPlayers
          }
          videogame {
            name
          }
          tournament {
            id
            name
            endAt
          }
          standings(query: { filter: { search: { searchString: $gamerTag}}}) {
            nodes {
              entrant {
                id
              }
              player {
                id
                user {
                  slug
                }
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

impl GQLData for SetGetterData {}

#[derive(Debug, Clone, Serialize)]
pub struct SetGetterVars {
    pub playerId: i32,
    page: i32,
    updatedAfter: Option<i64>,
    gamerTag: String,
}

impl GQLVars for SetGetterVars {
    fn update(&mut self) -> Self {
        panic!("internal error: something when wrong when updating SetGetterVars");
    }
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

    pub fn unpaginated_new(playerId: i32, updatedAfter: Option<i64>, gamerTag: &str) -> Self {
        Self {
            playerId,
            page: -999,
            updatedAfter,
            gamerTag: gamerTag.to_string(),
        }
    }
}

pub async fn make_set_getter_query(
    page: i32,
    usgv: Arc<Mutex<SetGetterVars>>,
) -> Result<SetGetterData> {
    let mut usgv_lock = usgv.lock().unwrap().clone();
    usgv_lock.page = page;
    let sgg = StartGG::connect();
    sgg.gql_client()
        .query_with_vars::<SetGetterData, SetGetterVars>(SET_GETTER_QUERY, usgv_lock)
        .await
        .transpose()
        .expect("an unexpected error occurred (empty data)")
        .map_err(|e| anyhow::anyhow!(e.message().to_string()))
}

#[cfg(test)]
mod tests {
    #![allow(unused)]    use std::sync::Arc;
    use std::sync::Mutex;

    use anyhow::Result;

    use crate::queries::set_getter::make_set_getter_query;
    use crate::queries::set_getter::SetGetterVars;

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    #[tokio::test]
    async fn set_getter() -> Result<()> {
        println!(
            "{:#?}",
            make_set_getter_query(
                1,
                Arc::new(Mutex::new(SetGetterVars::unpaginated_new(
                    DANTOTTO_PLAYER_ID,
                    None,
                    "Dantotto"
                )))
            )
            .await?
        );
        Ok(())
    }
}
