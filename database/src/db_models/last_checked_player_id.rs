#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.

use crate::schema::last_checked_player_id;

#[derive(Debug, Insertable, Queryable, QueryableByName)]
#[table_name = "last_checked_player_id"]
pub struct LastCheckedPlayerId {
    pub player_id: i32,
}

impl From<i32> for LastCheckedPlayerId {
    fn from(player_id: i32) -> Self {
        Self { player_id }
    }
}
