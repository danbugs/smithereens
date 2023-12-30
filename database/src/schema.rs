// @generated automatically by Diesel CLI.

diesel::table! {
    empty_player_ids (player_id) {
        player_id -> Int4,
    }
}

diesel::table! {
    error_logs (id) {
        id -> Int4,
        error_timestamp -> Timestamp,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    pidgtm_compile_times (time_in_seconds) {
        time_in_seconds -> Int4,
        calculation_timestamp -> Timestamp,
    }
}

diesel::table! {
    player_games (game_id, requester_id, set_id) {
        game_id -> Int4,
        requester_id -> Int4,
        requester_win -> Nullable<Bool>,
        order_num -> Int4,
        requester_char_played -> Nullable<Varchar>,
        opponent_char_played -> Nullable<Varchar>,
        stage -> Nullable<Varchar>,
        set_id -> Int4,
    }
}

diesel::table! {
    player_sets (id, requester_id) {
        id -> Int4,
        completed_at -> Int8,
        requester_id -> Int4,
        requester_tag_with_prefix -> Varchar,
        requester_score -> Int4,
        requester_seed -> Int4,
        opponent_tag_with_prefix -> Varchar,
        opponent_score -> Int4,
        opponent_seed -> Int4,
        result_type -> Int4,
        event_id -> Int4,
        tournament_id -> Int4,
        is_event_online -> Bool,
    }
}

diesel::table! {
    player_tournaments (tournament_id, event_id, requester_id) {
        tournament_id -> Int4,
        event_id -> Int4,
        event_name -> Varchar,
        tournament_name -> Varchar,
        end_at -> Int8,
        requester_id -> Int4,
        placement -> Int4,
        num_entrants -> Int4,
        seed -> Int4,
        link -> Varchar,
    }
}

diesel::table! {
    players (player_id) {
        player_id -> Int4,
        user_slug -> Varchar,
        prefix -> Nullable<Varchar>,
        gamer_tag -> Varchar,
        name -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        profile_picture -> Nullable<Varchar>,
        twitch_username -> Nullable<Varchar>,
        twitter_username -> Nullable<Varchar>,
        gender_pronouns -> Nullable<Varchar>,
        birthday -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        rankings -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    empty_player_ids,
    error_logs,
    pidgtm_compile_times,
    player_games,
    player_sets,
    player_tournaments,
    players,
);
