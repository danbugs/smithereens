use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeadToHeadResult {
    pub opponent_tag: String,
    pub total_sets: i64,
    pub wins: i64,
    pub losses: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ImageData {
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadSuccessResponse {
    pub message: String,
    pub filename: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadErrorResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum UploadResponse {
    Success(UploadSuccessResponse),
    Error(UploadErrorResponse),
}

#[derive(Serialize, Deserialize)]
pub struct CaptchaRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Player {
    pub player_id: i32,
    pub user_slug: String,
    pub prefix: Option<String>,
    pub gamer_tag: String,
    pub name: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub profile_picture: Option<String>,
    pub twitch_username: Option<String>,
    pub twitter_username: Option<String>,
    pub gender_pronouns: Option<String>,
    pub birthday: Option<String>,
    pub bio: Option<String>,
    pub rankings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tournament {
    pub tournament_id: i32,
    pub event_id: i32,
    pub tournament_name: String,
    pub event_name: String,
    pub end_at: i64,
    pub requester_id: i32,
    pub placement: i32,
    pub num_entrants: i32,
    pub seed: i32,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Set {
    pub id: i32,
    pub completed_at: i64,
    pub requester_id: i32,
    pub requester_tag_with_prefix: String,
    pub requester_score: i32,
    pub requester_seed: i32,
    pub opponent_tag_with_prefix: String,
    pub opponent_score: i32,
    pub opponent_seed: i32,
    pub result_type: i32,
    pub game_ids: Option<Vec<i32>>,
    pub event_id: i32,
    pub tournament_id: i32,
    pub is_event_online: bool,
}
