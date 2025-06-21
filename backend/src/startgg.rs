use axum::http; // StatusCode re‑export
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use thiserror::Error;
use tracing::instrument;

const FALLBACK_ENDPOINT: &str = "https://www.start.gg/api/-/gql";

#[derive(Debug, Error)]
pub enum StartggError {
    #[error("HTTP {0}")]
    Upstream(http::StatusCode),
    #[error("GraphQL errors: {0}")]
    GraphQl(String),
    #[error("Rate limited by start.gg API")]
    RateLimited,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),
}

#[derive(Clone, Debug)]
pub struct StartggClient {
    http: reqwest::Client,
    endpoint: String,
    headers: HeaderMap,
}

impl StartggClient {
    pub fn from_env() -> Result<Self, StartggError> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            std::env::var("SMITHE_STARTGG_TOKEN")?.parse().unwrap(),
        );
        headers.insert(
            "Client-Version",
            std::env::var("SMITHE_CLIENT_VERSION")
                .unwrap_or_else(|_| "20".into())
                .parse()
                .unwrap(),
        );

        let endpoint =
            std::env::var("SMITHE_STARTGG_ENDPOINT").unwrap_or_else(|_| FALLBACK_ENDPOINT.into());

        Ok(Self {
            http: reqwest::Client::new(),
            endpoint,
            headers,
        })
    }

    #[instrument(skip(self))]
    pub async fn query(&self, query: &str, variables: Value) -> Result<Value, StartggError> {
        let body = json!({ "query": query, "variables": variables });
        let resp = self
            .http
            .post(&self.endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?;

        let status = http::StatusCode::from_u16(resp.status().as_u16()).unwrap();
        let json: Value = resp.json().await?;

        if json.get("success") == Some(&Value::Bool(false))
            && json["message"].to_string().contains("Rate limit exceeded")
        {
            return Err(StartggError::RateLimited);
        }
        if let Some(errors) = json.get("errors") {
            return Err(StartggError::GraphQl(errors.to_string()));
        }
        if !status.is_success() {
            return Err(StartggError::Upstream(status));
        }
        Ok(json)
    }

    /// Query used by the front‑end search
    pub async fn search_players(
        &self,
        tag: &str,
        page: u32,
        per_page: u32,
    ) -> Result<Value, StartggError> {
        let gql = r#"query SearchByGamerTag($search: PlayerQuery!) {
        players(query: $search) {
          pageInfo { page totalPages }
          nodes {
            id prefix gamerTag url
            user {
              discriminator
              name
              location { country }
              images { type url }
            }
          }
        }
    }"#;

        let vars = json!({
            "search": {
              "filter": { "gamerTag": tag, "isUser": true, "hideTest": true },
              "page": page,
              "perPage": per_page
            }
        });

        self.query(gql, vars).await
    }

    pub async fn user_profile(&self, slug: &str, page: u32) -> Result<Value, StartggError> {
        let gql = r#"query Profile($slug: String!, $page: Int!) {
        user(slug: $slug) {
          id
          discriminator
          player { gamerTag prefix }
          location { city state country }
          images { type url }
          authorizations(types:[TWITTER,TWITCH,DISCORD]){
            externalUsername url
          }
          events(query:{
             page:$page, perPage:10, filter:{videogameId:1386}
          }){
            pageInfo { page totalPages }
            nodes {
              name slug numEntrants
              tournament { name }
            }
          }
        }
    }"#;

        let vars = json!({ "slug": slug, "page": page });
        self.query(gql, vars).await
    }

    /// All sets + games for one player in a specific event
    pub async fn player_event_sets(
        &self,
        user_id: i64,
        event_slug: &str,
    ) -> Result<Value, StartggError> {
        let gql = r#"query PlayerEventSets($userId: ID!, $eventSlug: String!) {
        event(slug: $eventSlug) {
          name
          slug
          numEntrants
          tournament { name }
          userEntrant(userId: $userId) {
            id
            name
            initialSeedNum
            standing { placement }
            paginatedSets(page: 1, perPage: 20, sortType: RECENT) {
              nodes {
                displayScore
                fullRoundText
                winnerId
                slots {
                  entrant {
                    id
                    name
                    participants { player { id } }
                  }
                }
                games {
                  winnerId
                  selections {
                    character { images(type:"stockIcon"){ url } }
                    entrant {
                      id
                      name
                      participants { player { id } }
                    }
                  }
                }
              }
            }
          }
        }
    }"#;

        let vars = json!({ "userId": user_id, "eventSlug": event_slug });
        self.query(gql, vars).await
    }

    /// Lightweight query for health‑checks / rate‑limit testing.
    /// `#[allow(dead_code)]` because not used in the main app (only in tests).
    #[allow(dead_code)]
    pub async fn get_event_id(&self, slug: &str) -> Result<Value, StartggError> {
        let gql = r#"query getEventId($slug: String) {
            event(slug: $slug) { id name }
        }"#;
        let vars = json!({ "slug": slug });
        self.query(gql, vars).await
    }
}
