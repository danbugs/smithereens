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
        let gamer_tag_with_prefix = if ppgd.player.prefix.is_empty() {
            ppgd.player.gamerTag
        } else {
            format!("{} | {}", ppgd.player.prefix, ppgd.player.gamerTag)
        };
        
        Self {
            player_id: ppgd.player.id,
            gamer_tag_with_prefix,
            user_slug: ppgd.player.user.slug,
        }
    }
}
