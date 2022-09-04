#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::player_tournaments;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "player_tournaments"]
pub struct Tournament {
    pub tournament_id: i32,
    event_id: i32,
    event_at_tournament: String,
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

    fn ne(&self, other: &Self) -> bool {
        self.tournament_id != other.tournament_id
            || self.event_id != other.event_id
            || self.requester_id != other.requester_id
    }
}

impl Tournament {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tid: i32,
        eid: i32,
        event_name: &str,
        tournament_name: &str,
        rid: i32,
        p: i32,
        ne: i32,
        s: i32,
        l: &str,
    ) -> Self {
        Self {
            tournament_id: tid,
            event_id: eid,
            event_at_tournament: format!("{} @ {}", event_name, tournament_name),
            requester_id: rid,
            placement: p,
            num_entrants: ne,
            seed: s,
            link: l.to_string(),
        }
    }
}
