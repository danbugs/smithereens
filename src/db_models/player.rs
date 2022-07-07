use crate::queries::player_getter::PIDGTM_PlayerGetterData;
use crate::schema::players;

#[derive(Debug, Insertable, Queryable)]
#[table_name="players"]
pub struct Player {
    pub player_id: i32,
    pub gamer_tag_with_prefix: String,
    pub user_slug: String,
}

impl From<PIDGTM_PlayerGetterData> for Player {
    fn from(ppgd: PIDGTM_PlayerGetterData) -> Self {
        Self {
            player_id: ppgd.player.id,
            gamer_tag_with_prefix: ppgd.player.gamerTag,
            user_slug: ppgd.player.user.slug,
        }
    }
}
