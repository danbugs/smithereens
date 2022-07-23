#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

pub const STARTGG_ENDPOINT: &str = "https://api.start.gg/gql/alpha";

#[derive(Debug, Deserialize)]
pub struct Phase {
    pub id: Option<i32>,
    pub seeds: Option<SeedConnection>,
}

#[derive(Debug, Deserialize)]
pub struct SeedConnection {
    pub nodes: Vec<Seed>,
}

#[derive(Debug, Deserialize)]
pub struct Seed {
    seedNum: i32,
    entrant: Entrant,
}

#[derive(Debug, Deserialize)]
pub struct Entrant {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub slug: String,
    pub phases: Vec<Phase>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub prefix: String,
    pub gamerTag: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    slug: String,
}

pub struct StartGG {
    gql_client: gql_client::Client,
}

impl StartGG {
    pub fn connect() -> Self {
        let mut headers = HashMap::new();
        let bearer_token = concat!("Bearer ", env!("STARTGG_TOKEN"));
        headers.insert("authorization", bearer_token);
        Self {
            gql_client: gql_client::Client::new_with_headers(STARTGG_ENDPOINT, headers),
        }
    }

    pub fn gql_client(&self) -> gql_client::Client {
        self.gql_client.clone()
    }
}
