#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::sets;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "sets"]
pub struct Set {
    id: i32,
    completed_at: i64,
    requester_id: i32,
    requester_tag_with_prefix: String,
    requester_score: i32,
    opponent_tag_with_prefix: String,
    opponent_score: i32,
    result_type: i32,
    event_at_tournament: String,
    is_event_online: bool
}
