#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::{schema::player_games, startgg::SSBU_CHARACTERS};

#[derive(Debug, Insertable, Queryable)]
#[table_name = "player_games"]
pub struct Game {
    game_id: i32,
    requester_id: i32,
    requester_win: bool,
    order_num: i32,
    requester_char_played: Option<String>,
    opponent_char_played: Option<String>,
    stage: Option<String>,
}

impl Game {
    pub fn new(
        gid: i32,
        rid: i32,
        rw: bool,
        onum: i32,
        rcp_num_o: Option<i32>,
        ocp_num_o: Option<i32>,
        s: Option<String>,
    ) -> Self {
        Self {
            game_id: gid,
            requester_id: rid,
            requester_win: rw,
            order_num: onum,
            requester_char_played: if let Some(rcp_num) = rcp_num_o {
                Some(get_character_from_id(rcp_num))
            } else {
                None
            },
            opponent_char_played: if let Some(ocp_num) = ocp_num_o {
                Some(get_character_from_id(ocp_num))
            } else {
                None
            },
            stage: s,
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
