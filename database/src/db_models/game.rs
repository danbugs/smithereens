#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::player_games;
use startgg::SSBU_CHARACTERS;

#[derive(Debug, Clone, Insertable, Queryable, QueryableByName)]
#[table_name = "player_games"]
pub struct Game {
    pub game_id: i32,
    requester_id: i32,
    requester_win: Option<bool>,
    order_num: i32,
    requester_char_played: Option<String>,
    opponent_char_played: Option<String>,
    stage: Option<String>,
    set_id: i32,
}

impl Game {
    pub fn new(
        gid: i32,
        rid: i32,
        rw: Option<bool>,
        onum: i32,
        rcp_num_o: Option<i32>,
        ocp_num_o: Option<i32>,
        s: Option<String>,
        sid: i32,
    ) -> Self {
        Self {
            game_id: gid,
            requester_id: rid,
            requester_win: rw,
            order_num: onum,
            requester_char_played: rcp_num_o.map(get_character_from_id),
            opponent_char_played: ocp_num_o.map(get_character_from_id),
            stage: s,
            set_id: sid,
        }
    }
}

fn get_character_from_id(id: i32) -> String {
    SSBU_CHARACTERS
        .iter()
        .find(|i| i.0.eq(&id))
        .unwrap()
        .1
        .to_string()
}
