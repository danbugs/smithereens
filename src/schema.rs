table! {
    empty_player_ids (player_id) {
        player_id -> Int4,
    }
}

table! {
    last_checked_player_id (player_id) {
        player_id -> Int4,
    }
}

table! {
    players (player_id) {
        player_id -> Int4,
        gamer_tag_with_prefix -> Varchar,
        user_slug -> Varchar,
    }
}

table! {
    sets (id, requester_id) {
        id -> Int4,
        completed_at -> Int8,
        requester_id -> Int4,
        requester_tag_with_prefix -> Varchar,
        requester_score -> Int4,
        opponent_tag_with_prefix -> Varchar,
        opponent_score -> Int4,
        result_type -> Int4,
        event_at_tournament -> Varchar,
        is_event_online -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    empty_player_ids,
    last_checked_player_id,
    players,
    sets,
);
