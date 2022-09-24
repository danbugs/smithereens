CREATE TABLE player_tournaments(
    tournament_id INTEGER,
    event_id INTEGER,
    event_name VARCHAR NOT NULL,
    tournament_name VARCHAR NOT NULL,
    end_at BIGINT NOT NULL,
    requester_id INTEGER,
    placement INTEGER NOT NULL,
    num_entrants INTEGER NOT NULL,
    seed INTEGER NOT NULL,
    link VARCHAR NOT NULL,
    PRIMARY KEY(tournament_id, event_id, requester_id)
);

CREATE TABLE player_sets(
    id INTEGER,
    completed_at BIGINT NOT NULL,
    requester_id INTEGER,
    requester_tag_with_prefix VARCHAR NOT NULL,
    requester_score INTEGER NOT NULL,
    requester_seed INTEGER NOT NULL,
    opponent_tag_with_prefix VARCHAR NOT NULL,
    opponent_score INTEGER NOT NULL,
    opponent_seed INTEGER NOT NULL,
    result_type INTEGER NOT NULL,
    game_ids INTEGER ARRAY,
    event_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    is_event_online BOOLEAN NOT NULL,
    PRIMARY KEY(id, requester_id)
);

CREATE TABLE player_games(
    game_id INTEGER,
    requester_id INTEGER,
    requester_win BOOLEAN NOT NULL,
    order_num INTEGER NOT NULL,
    requester_char_played VARCHAR,
    opponent_char_played VARCHAR,
    stage VARCHAR,
    PRIMARY KEY(game_id, requester_id)
);