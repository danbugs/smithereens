#![allow(non_snake_case)]

use crate::startgg::{Phase, Seed, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SEEDING_GETTER_QUERY: &str = r#"
query SeedingGetter($phaseId: ID!, $page: Int!, $perPage: Int = 500) {
    phase(id:$phaseId) {
      seeds(query: {
        page: $page
        perPage: $perPage
        sortBy: "seedNum"
      }){
        nodes {
          seedNum
          entrant {
            name
          }
        }
      }
    }
  }
"#;

// ^^^ 500 is the maximum allowed perPage

#[derive(Debug, Deserialize)]
pub struct SeedingGetterData {
    phase: Phase,
}

#[derive(Serialize)]
pub struct SeedingGetterVars {
    phaseId: i32,
    page: i32,
    perPage: i32,
}

impl SeedingGetterVars {
    pub fn new(phaseId: i32, page: i32, perPage: Option<i32>) -> Self {
        let perPage = if perPage.is_none() {
            Some(500)
        } else {
            perPage
        }
        .unwrap(); // this unwrap can't fail, so it's fine

        Self {
            phaseId,
            page,
            perPage,
        }
    }
}

pub async fn make_seeding_getter_query(phase_id: i32) -> Result<Vec<Seed>> {
    let sgg = StartGG::connect();
    let mut overall = vec![];
    let mut curr_page = 1;

    while let Some(mut seeds) = sgg
        .gql_client()
        .query_with_vars::<SeedingGetterData, SeedingGetterVars>(
            SEEDING_GETTER_QUERY,
            SeedingGetterVars::new(phase_id, curr_page, None),
        )
        .await
        .map_err(|_| anyhow::anyhow!("failed to get entrants under phase id '{}'", phase_id))?
        .ok_or_else(|| anyhow::anyhow!("no event found for specified slug: '{}'", phase_id))?
        .phase
        .seeds
        .and_then(|s| if s.nodes.is_empty() { None } else { Some(s) })
    {
        overall.append(&mut seeds.nodes);
        curr_page += 1;
    }

    Ok(overall)
}

#[cfg(test)]
mod tests {
    use crate::{
        queries::seeding_geeter::{SeedingGetterData, SeedingGetterVars, SEEDING_GETTER_QUERY},
        startgg::StartGG,
    };
    use anyhow::Result;

    const BOBC4_VAR_PHASEID: i32 = 1078793; // pools phase ID to get overall seeding
    const BOBC4_VAR_PAGE: i32 = 1; // avoiding pagination
    const BOBC4_VAR_PERPAGE: i32 = 8; // just 8 to not clutter stdout

    // note: it is viable to get 1 page in events up to 500 people

    #[tokio::test]
    async fn seeding_getter() -> Result<()> {
        let sgg = StartGG::connect();
        let vars =
            SeedingGetterVars::new(BOBC4_VAR_PHASEID, BOBC4_VAR_PAGE, Some(BOBC4_VAR_PERPAGE));

        let nodes = sgg
            .gql_client()
            .query_with_vars::<SeedingGetterData, SeedingGetterVars>(SEEDING_GETTER_QUERY, vars)
            .await
            .unwrap();

        dbg!(nodes);

        Ok(())
    }
}
