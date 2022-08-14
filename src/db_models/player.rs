#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::players;
use crate::startgg;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "players"]
pub struct Player {
    pub player_id: i32,
    pub gamer_tag_with_prefix: String,
    pub user_slug: String,
}

impl From<startgg::Player> for Player {
    fn from(p: startgg::Player) -> Self {
        // ^^^ if we got here, player is guaranteed to be Ok(..)
        let gamer_tag_with_prefix = if p.prefix.is_none() || p.prefix.as_ref().unwrap().is_empty() {
            // ^^^ it is ok to unwrap here due to the first conditional
            p.gamerTag
        } else {
            format!("{} | {}", p.prefix.unwrap(), p.gamerTag)
            // ^^^ it is ok to unwrap here because already we know it is not None
        };

        Self {
            player_id: p.id,
            gamer_tag_with_prefix,
            user_slug: p.user.unwrap().slug,
            // ^^^ ok to be unwrapping, afaik, only test account don't have a user slug
            // associated with them, and we should be catching those before we get here
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.gamer_tag_with_prefix)
    }
}
