use crate::schema::player_page_views;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = player_page_views)]
pub struct NewPlayerPageView {
    pub access_timestamp: NaiveDateTime,
    pub player_id: i32,
}

impl NewPlayerPageView {
    pub fn new(player_id: i32) -> Self {
        Self {
            access_timestamp: chrono::Utc::now().naive_utc(),
            player_id,
        }
    }
}
