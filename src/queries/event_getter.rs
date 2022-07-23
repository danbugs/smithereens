#![allow(non_snake_case)]

use crate::startgg::{Event, StartGG};
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

#[derive(Serialize)]
pub struct EventGetterVars<'tegv> {
    slug: &'tegv str,
}

impl<'tegv> EventGetterVars<'tegv> {
    pub fn new(slug: &'tegv str) -> Self {
        Self { slug }
    }
}

pub async fn make_event_getter_query(event_slug: &str) -> Result<EventGetterData> {
    let sgg = StartGG::connect();
    let vars = EventGetterVars::new(event_slug);
    sgg.gql_client()
        .query_with_vars::<EventGetterData, EventGetterVars>(EVENTS_GETTER_QUERY, vars)
        .await
        .map_err(|_| anyhow::anyhow!("failed to get events under the '{}' event slug", event_slug))?
        .ok_or_else(|| anyhow::anyhow!("no event found for specified slug: '{}'", event_slug))
}

pub async fn get_phase_id_from_event_slug(event_slug: &str) -> Result<i32> {
    make_event_getter_query(event_slug).await?.event.phases[0]
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
    use anyhow::Result;

    use crate::queries::event_getter::make_event_getter_query;

    const BOBC4_VAR_SLUG: &str = "tournament/battle-of-bc-4-2/event/ultimate-singles-bracket";

    #[tokio::test]
    async fn event_getter() -> Result<()> {
        dbg!(make_event_getter_query(BOBC4_VAR_SLUG).await?);
        Ok(())
    }
}
