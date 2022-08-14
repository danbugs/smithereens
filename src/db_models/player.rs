#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::queries::player_getter::PIDGTM_PlayerGetterData;
use crate::schema::players;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "players"]
pub struct Player {
    pub player_id: i32,
    pub gamer_tag_with_prefix: String,
    pub user_slug: String,
}

impl From<PIDGTM_PlayerGetterData> for Player {
    fn from(ppgd: PIDGTM_PlayerGetterData) -> Self {
        let gamer_tag_with_prefix =
            if ppgd.player.prefix.is_none() || ppgd.player.prefix.as_ref().unwrap().is_empty() {
                // ^^^ it is ok to unwrap here due to the first conditional
                ppgd.player.gamerTag
            } else {
                format!("{} | {}", ppgd.player.prefix.as_ref().unwrap(), ppgd.player.gamerTag)
                // ^^^ it is ok to unwrap here because already we know it is not none
            };

        Self {
            player_id: ppgd.player.id,
            gamer_tag_with_prefix,
            user_slug: ppgd.player.user.unwrap().slug,
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
