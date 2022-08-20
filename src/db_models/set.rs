#![allow(clippy::extra_unused_lifetimes)]
// ^^^ this is needed& because Insertable introduces a lifetime we don't use
// — an auto fix for this exists only in Diesel v2.
use crate::schema::sets;

use anyhow::Result;
use regex::Regex;

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
    is_event_online: bool,
}

impl Set {
    pub fn new(
        id: i32,
        cat: i64,
        rid: i32,
        gtag: &str,
        dscore: &str,
        ename: &str,
        tname: &str,
        is_on: bool,
    ) -> Self {
        let (tag1, score1, tag2, score2) = lex_score(dscore).expect("failed to lex display score");
        let ((rtag, rscore), (otag, oscore)) =
            separate_requestor_and_opponent_data(gtag, &tag1, score1, &tag2, score2);
        let result_type = determine_result_type(dscore, rscore, oscore);

        Self {
            id,
            completed_at: cat,
            requester_id: rid,
            requester_tag_with_prefix: rtag,
            requester_score: rscore,
            opponent_tag_with_prefix: otag,
            opponent_score: oscore,
            result_type,
            event_at_tournament: format!("{} @ {}", ename, tname),
            is_event_online: is_on,
        }
    }
}

fn lex_score(dscore: &str) -> Result<(String, i32, String, i32)> {
    let re = Regex::new(r"^(.+)\s(\d)\s-\s(.+)\s(\d)$")?;
    let caps = re.captures(dscore).expect("failed to regex display score");
    Ok((
        caps.get(1).unwrap().as_str().to_string(),
        caps.get(2)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap(),
        caps.get(3).unwrap().as_str().to_string(),
        caps.get(4)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap(),
    ))
}

fn separate_requestor_and_opponent_data(
    gtag: &str,
    tag1: &str,
    score1: i32,
    tag2: &str,
    score2: i32,
) -> ((String, i32), (String, i32)) {
    if tag1.contains(gtag) {
        ((tag1.to_string(), score1), (tag2.to_string(), score2))
    } else {
        ((tag2.to_string(), score2), (tag1.to_string(), score1))
    }
}

fn determine_result_type(dscore: &str, rscore: i32, oscore: i32) -> i32 {
    if rscore == -1 {
        -1
    } else if oscore == -1 {
        1
    } else if rscore > oscore {
        2
    } else if rscore < oscore {
        -2
    } else {
        panic!("unrecognizable result type for set: {}", dscore);
    }
}
