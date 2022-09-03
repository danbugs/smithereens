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
    player_games (game_id, requester_id) {
        game_id -> Int4,
        requester_id -> Int4,
        order_num -> Int4,
        requester_char_played -> Varchar,
        opponent_char_played -> Varchar,
        stage -> Varchar,
    }
}

table! {
    player_sets (id, requester_id) {
        id -> Int4,
        completed_at -> Int8,
        requester_id -> Int4,
        requester_tag_with_prefix -> Varchar,
        requester_score -> Int4,
        opponent_tag_with_prefix -> Varchar,
        opponent_score -> Int4,
        result_type -> Int4,
        game_ids -> Nullable<Array<Int4>>,
        event_id -> Int4,
        tournament_id -> Int4,
        is_event_online -> Bool,
    }
}

table! {
    player_tournaments (tournament_id, event_id, requester_id) {
        tournament_id -> Int4,
        event_id -> Int4,
        event_at_tournament -> Varchar,
        requester_id -> Int4,
        placement -> Int4,
        num_entrants -> Int4,
        seed -> Int4,
    }
}

table! {
    players (player_id) {
        player_id -> Int4,
        gamer_tag_with_prefix -> Varchar,
        user_slug -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    empty_player_ids,
    last_checked_player_id,
    player_games,
    player_sets,
    player_tournaments,
    players,
);
