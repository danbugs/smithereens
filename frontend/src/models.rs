use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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