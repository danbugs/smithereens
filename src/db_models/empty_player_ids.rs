#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.

use crate::schema::empty_player_ids;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "empty_player_ids"]
pub struct EmptyPlayerId {
    pub player_id: i32,
}

impl From<i32> for EmptyPlayerId {
    fn from(player_id: i32) -> Self {
        Self { player_id }
    }
}
