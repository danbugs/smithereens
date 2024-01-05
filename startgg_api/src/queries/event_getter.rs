#![allow(non_snake_case)]

use crate::{Event, GQLData, StartGG};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const EVENTS_GETTER_QUERY: &str = r#"
query TournamentEventsGetter($slug: String!) {
  event(slug: $slug) {
    slug 
    phases {
      id
    }
  }
}
"#;

#[derive(Debug, Deserialize)]
pub struct EventGetterData {
    event: Event,
}

impl GQLData for EventGetterData {}

#[derive(Debug, Serialize)]
pub struct EventGetterVars {
    slug: String,
}

// maybe impl GQLVars

impl EventGetterVars {
    pub fn new(slug: &str) -> Self {
        Self {
            slug: slug.to_string(),
        }
    }
}

pub async fn make_event_getter_query(event_slug: &str) -> Result<EventGetterData> {
    let sgg = StartGG::connect();
    let vars = EventGetterVars::new(event_slug);
    sgg.gql_client()
        .query_with_vars::<EventGetterData, EventGetterVars>(EVENTS_GETTER_QUERY, vars)
        .await
        .map_err(|e| anyhow::anyhow!(e.message().to_string()))?
        .ok_or_else(|| anyhow::anyhow!("no event found for specified slug: '{}'", event_slug))
}

pub async fn get_phase_id_from_event_slug(event_slug: &str) -> Result<i32> {
    // vvv fine to unwrap given context
    make_event_getter_query(event_slug)
        .await?
        .event
        .phases
        .unwrap()[0]
        .id
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no phases in event under the specific slug: '{}'",
                event_slug
            )
        }) // get first phase as it provides overall seeding
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use anyhow::Result;

    use crate::queries::event_getter::make_event_getter_query;

    const BOBC4_VAR_SLUG: &str = "tournament/battle-of-bc-4-2/event/ultimate-singles-bracket";

    #[tokio::test]
    async fn event_getter() -> Result<()> {
        println!("{:#?}", make_event_getter_query(BOBC4_VAR_SLUG).await?);
        Ok(())
    }
}
