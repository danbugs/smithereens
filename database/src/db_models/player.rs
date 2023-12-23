#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::players;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, QueryableByName)]
#[table_name = "players"]
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

impl From<startgg::Player> for Player {
    fn from(p: startgg::Player) -> Self {
        // ^^^ if we got here, player is guaranteed to be Ok(..)

        let u = p.user.unwrap();

        let (s, c) = if let Some(l) = u.clone().location {
            (l.state.clone(), l.country)
        } else {
            (None, None)
        };

        Self {
            player_id: p.id,
            user_slug: u.clone().slug.unwrap(),
            // ^^^ ok to be unwrapping, afaik, only test account don't have a user slug
            // associated with them, and we should be catching those before we get here
            prefix: p.prefix,
            gamer_tag: p.gamerTag.unwrap(),
            name: u.clone().name,
            state: s,
            country: c,
            profile_picture: if u.images.is_some() && !u.clone().images.unwrap().is_empty() {
                u.clone().images.map(|i| i[0].url.clone())
            } else {
                None
            },
            twitch_username: if let Some(a) = u.clone().authorizations {
                a.iter()
                    .find(|a| a.r#type == "TWITCH")
                    .and_then(|twitch| twitch.externalUsername.clone())
            } else {
                None
            },
            twitter_username: if let Some(a) = u.clone().authorizations {
                a.iter()
                    .find(|a| a.r#type == "TWITTER")
                    .and_then(|twitter| twitter.externalUsername.clone())
            } else {
                None
            },
            gender_pronouns: u.clone().genderPronoun,
            birthday: u.clone().birthday,
            bio: u.bio,
            rankings: p.rankings.map(|r| {
                r.iter()
                    .map(|pr| format!("#{} @ {}", pr.rank, pr.title))
                    .collect::<Vec<String>>()
            }),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gamer_tag_with_prefix =
            if self.prefix.is_none() || self.prefix.as_ref().unwrap().is_empty() {
                // ^^^ it is ok to unwrap here due to the first conditional
                self.gamer_tag.clone()
            } else {
                format!("({} | {}", self.prefix.as_ref().unwrap(), &self.gamer_tag)
            };
        write!(f, "{}", gamer_tag_with_prefix)
    }
}
