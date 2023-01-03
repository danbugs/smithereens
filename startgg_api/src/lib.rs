#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod queries;

use std::collections::HashMap;

use as_any::AsAny;
use serde::Deserialize;

pub const STARTGG_ENDPOINT: &str = "https://api.start.gg/gql/alpha";

pub trait GQLData: AsAny + std::fmt::Debug {}

pub trait GQLVars: AsAny + std::fmt::Debug {
    fn update(&mut self) -> Self;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Phase {
    pub id: Option<i32>,
    pub seeds: Option<SeedConnection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeedConnection {
    pub nodes: Vec<Seed>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Seed {
    pub seedNum: i32,
    entrant: Option<Entrant>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entrant {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub phases: Option<Vec<Phase>>,
    pub name: Option<String>,
    pub numEntrants: Option<i32>,
    pub isOnline: Option<bool>,
    pub videogame: Option<Videogame>,
    pub tournament: Option<Tournament>,
    pub standings: Option<StandingConnection>,
    pub teamRosterSize: Option<TeamRosterSize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeamRosterSize {
    maxPlayers: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandingConnection {
    pub nodes: Vec<Standing>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Videogame {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub id: i32,
    pub prefix: Option<String>,
    // ^^^ if the player never changed their
    // prefix before, it will be null.
    // If the player changed it but removed it,
    // it will be an empty string.
    pub gamerTag: Option<String>,
    pub user: Option<User>,
    // ^^^ test accounts are not associated
    // to a user, they have a null user ~
    // we don't want to fail here.
    pub rankings: Option<Vec<PlayerRank>>,
    pub sets: Option<SetConnection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerRank {
    pub rank: i32,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetConnection {
    pub nodes: Vec<Set>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub endAt: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Set {
    pub id: i32,
    pub games: Option<Vec<Game>>,
    pub slots: Vec<SetSlot>,
    pub completedAt: i64,
    pub phaseGroup: PhaseGroup,
    pub event: Event,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetSlot {
    pub entrant: Entrant,
    pub seed: Seed,
    pub standing: Standing,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Standing {
    pub entrant: Option<Entrant>,
    pub player: Option<Player>,
    pub stats: Option<StandingStats>,
    pub placement: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandingStats {
    pub score: Score,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Score {
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    pub id: i32,
    pub winnerId: Option<i32>,
    pub orderNum: i32,
    pub selections: Option<Vec<GameSelection>>,
    pub stage: Option<Stage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameSelection {
    pub entrant: Entrant,
    pub selectionValue: i32, // this will be an i32 that represents the character
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhaseGroup {
    pub bracketType: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub location: Option<Address>,
    pub bio: Option<String>,
    pub birthday: Option<String>,
    pub images: Option<Vec<Image>>,
    pub slug: Option<String>,
    pub genderPronoun: Option<String>,
    pub authorizations: Option<Vec<ProfileAuthorization>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileAuthorization {
    pub externalUsername: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Address {
    pub state: Option<String>,
    pub country: Option<String>,
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

pub const SSBU_CHARACTERS: [(i32, &str); 87] = [
    (1271, "Bayonetta"),
    (1272, "Bowser Jr."),
    (1273, "Bowser"),
    (1274, "Captain Falcon"),
    (1275, "Cloud"),
    (1276, "Corrin"),
    (1277, "Daisy"),
    (1278, "Dark Pit"),
    (1279, "Diddy Kong"),
    (1280, "Donkey Kong"),
    (1282, "Dr. Mario"),
    (1283, "Duck Hunt"),
    (1285, "Falco"),
    (1286, "Fox"),
    (1287, "Ganondorf"),
    (1289, "Greninja"),
    (1290, "Ice Climbers"),
    (1291, "Ike"),
    (1292, "Inkling"),
    (1293, "Jigglypuff"),
    (1294, "King Dedede"),
    (1295, "Kirby"),
    (1296, "Link"),
    (1297, "Little Mac"),
    (1298, "Lucario"),
    (1299, "Lucas"),
    (1300, "Lucina"),
    (1301, "Luigi"),
    (1302, "Mario"),
    (1304, "Marth"),
    (1305, "Mega Man"),
    (1307, "Meta Knight"),
    (1310, "Mewtwo"),
    (1311, "Mii Brawler"),
    (1313, "Ness"),
    (1314, "Olimar"),
    (1315, "Pac-Man"),
    (1316, "Palutena"),
    (1317, "Peach"),
    (1318, "Pichu"),
    (1319, "Pikachu"),
    (1320, "Pit"),
    (1321, "Pokemon Trainer"),
    (1322, "Ridley"),
    (1323, "R.O.B."),
    (1324, "Robin"),
    (1325, "Rosalina"),
    (1326, "Roy"),
    (1327, "Ryu"),
    (1328, "Samus"),
    (1329, "Sheik"),
    (1330, "Shulk"),
    (1331, "Snake"),
    (1332, "Sonic"),
    (1333, "Toon Link"),
    (1334, "Villager"),
    (1335, "Wario"),
    (1336, "Wii Fit Trainer"),
    (1337, "Wolf"),
    (1338, "Yoshi"),
    (1339, "Young Link"),
    (1340, "Zelda"),
    (1341, "Zero Suit Samus"),
    (1405, "Mr. Game & Watch"),
    (1406, "Incineroar"),
    (1407, "King K. Rool"),
    (1408, "Dark Samus"),
    (1409, "Chrom"),
    (1410, "Ken"),
    (1411, "Simon Belmont"),
    (1412, "Richter"),
    (1413, "Isabelle"),
    (1414, "Mii Swordfighter"),
    (1415, "Mii Gunner"),
    (1441, "Piranha Plant"),
    (1453, "Joker"),
    (1526, "Hero"),
    (1530, "Banjo-Kazooie"),
    (1532, "Terry"),
    (1539, "Byleth"),
    (1746, "Random Character"),
    (1747, "Min Min"),
    (1766, "Steve"),
    (1777, "Sephiroth"),
    (1795, "Pyra & Mythra"),
    (1846, "Kazuya"),
    (1897, "Sora"),
];
