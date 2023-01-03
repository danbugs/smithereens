#![allow(clippy::extra_unused_lifetimes)]
use serde::Serialize;

// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::player_tournaments;

#[derive(Debug, Insertable, Queryable, Serialize)]
#[table_name = "player_tournaments"]
pub struct Tournament {
    pub tournament_id: i32,
    event_id: i32,
    tournament_name: String,
    event_name: String,
    end_at: i64,
    requester_id: i32,
    placement: i32,
    num_entrants: i32,
    seed: i32,
    link: String,
}

impl PartialEq for Tournament {
    fn eq(&self, other: &Self) -> bool {
        self.tournament_id == other.tournament_id
            && self.event_id == other.event_id
            && self.requester_id == other.requester_id
    }
}

impl Tournament {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tid: i32,
        eid: i32,
        ename: &str,
        tname: &str,
        eat: i64,
        rid: i32,
        p: i32,
        ne: i32,
        s: i32,
        l: &str,
    ) -> Self {
        Self {
            tournament_id: tid,
            event_id: eid,
            event_name: ename.to_string(),
            tournament_name: tname.to_string(),
            requester_id: rid,
            end_at: eat,
            placement: p,
            num_entrants: ne,
            seed: s,
            link: l.to_string(),
        }
    }
}
