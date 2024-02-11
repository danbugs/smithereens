use crate::schema::player_games;
use startgg::SSBU_CHARACTERS;

#[derive(Debug, Clone, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = player_games)]
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
    #[allow(clippy::too_many_arguments)]
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
            requester_char_played: rcp_num_o.and_then(get_character_from_id),
            opponent_char_played: ocp_num_o.and_then(get_character_from_id),
            stage: s,
            set_id: sid,
        }
    }
}

fn get_character_from_id(id: i32) -> Option<String> {
    SSBU_CHARACTERS.iter().find(|i| i.0.eq(&id)).map(|char| char.1.to_string())
}
