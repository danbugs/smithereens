#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{GQLData, GQLVars, Player, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const PIDGTM_PLAYER_GETTER_QUERY: &str = r#"
query PIDGTM_PlayerGetter($playerId: ID!) {
    player(id: $playerId) {
      id
      prefix
      gamerTag
      rankings {
        rank
        title
      }
      user {
        name
        location {
          state
          country
        }
        bio
        birthday
        images(type: "profile") {
          url
        }
        slug
        genderPronoun
        authorizations(types: [TWITCH, TWITTER]) {
          externalUsername
          type
        }
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

#[derive(Debug, Clone, Serialize)]
pub struct PIDGTM_PlayerGetterVars {
    pub playerId: i32,
}

impl GQLVars for PIDGTM_PlayerGetterVars {
    fn update(&mut self) -> Self {
        sleep(Duration::from_secs(60));
        self.clone()
    }
}

impl PIDGTM_PlayerGetterVars {
    pub fn empty() -> Self {
        Self { playerId: 0 }
    }
}

pub async fn make_pidgtm_player_getter_query(
    player_id: i32,
    upgv: Arc<Mutex<PIDGTM_PlayerGetterVars>>,
) -> Result<PIDGTM_PlayerGetterData> {
    let mut upgv_lock = upgv.lock().unwrap().clone();
    upgv_lock.playerId = player_id;
    let sgg = StartGG::connect();
    sgg.gql_client()
        .query_with_vars::<PIDGTM_PlayerGetterData, PIDGTM_PlayerGetterVars>(
            PIDGTM_PLAYER_GETTER_QUERY,
            upgv_lock,
        )
        .await
        .transpose()
        .expect("an unexpected error occurred (empty data)")
        .map_err(|e| anyhow::anyhow!(e.message().to_string()))
}

#[cfg(test)]
mod tests {
    #![allow(unused)]    use std::sync::{Arc, Mutex};

    use anyhow::Result;

    use crate::queries::player_getter::{make_pidgtm_player_getter_query, PIDGTM_PlayerGetterVars};

    const DANTOTTO_PLAYER_ID: i32 = 1178271;

    #[tokio::test]
    async fn player_getter() -> Result<()> {
        println!(
            "{:#?}",
            make_pidgtm_player_getter_query(
                DANTOTTO_PLAYER_ID,
                Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty()))
            )
            .await?
        );
        Ok(())
    }
}
