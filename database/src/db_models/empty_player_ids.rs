use crate::schema::empty_player_ids;

#[derive(Debug, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = empty_player_ids)]
pub struct EmptyPlayerId {
    pub player_id: i32,
}

impl From<i32> for EmptyPlayerId {
    fn from(player_id: i32) -> Self {
        Self { player_id }
    }
}
