#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::player_sets;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "player_sets"]
pub struct Set {
    id: i32,
    pub completed_at: i64,
    requester_id: i32,
    requester_tag_with_prefix: String,
    requester_score: i32,
    requester_seed: i32,
    opponent_tag_with_prefix: String,
    opponent_score: i32,
    opponent_seed: i32,
    result_type: i32,
    game_ids: Option<Vec<i32>>,
    event_id: i32,
    tournament_id: i32,
    is_event_online: bool,
}

impl Set {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        cat: i64,
        rid: i32,
        is_on: bool,
        e_id: i32,
        t_id: i32,
        gids: Option<Vec<i32>>,
        rtag: &str,
        rscore: i32,
        rseed: i32,
        otag: &str,
        oscore: i32,
        oseed: i32,
    ) -> Self {
        let result_type = determine_result_type(rscore, oscore);

        Self {
            id,
            completed_at: cat,
            requester_id: rid,
            requester_tag_with_prefix: rtag.to_string(),
            requester_score: rscore,
            requester_seed: rseed,
            opponent_tag_with_prefix: otag.to_string(),
            opponent_score: oscore,
            opponent_seed: oseed,
            result_type,
            is_event_online: is_on,
            event_id: e_id,
            tournament_id: t_id,
            game_ids: gids,
        }
    }
}

fn determine_result_type(rscore: i32, oscore: i32) -> i32 {
    if rscore == -1 {
        -1
    } else if oscore == -1 {
        1
    } else if rscore > oscore {
        2
    } else if rscore < oscore {
        -2
    } else {
        panic!("unrecognizable result type for set");
    }
}
