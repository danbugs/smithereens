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

allow_tables_to_appear_in_same_query!(empty_player_ids, last_checked_player_id, players,);
