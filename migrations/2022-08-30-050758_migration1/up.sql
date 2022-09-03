CREATE TABLE player_tournaments(
    tournament_id INTEGER,
    event_id INTEGER,
    event_at_tournament VARCHAR NOT NULL,
    requester_id INTEGER,
    placement INTEGER NOT NULL,
    num_entrants INTEGER NOT NULL,
    seed INTEGER NOT NULL,
    PRIMARY KEY(tournament_id, event_id, requester_id)
);

CREATE TABLE player_sets(
    id INTEGER,
    completed_at BIGINT NOT NULL,
    requester_id INTEGER,
    requester_tag_with_prefix VARCHAR NOT NULL,
    requester_score INTEGER NOT NULL,
    opponent_tag_with_prefix VARCHAR NOT NULL,
    opponent_score INTEGER NOT NULL,
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
    order_num INTEGER NOT NULL,
    requester_char_played VARCHAR NOT NULL,
    opponent_char_played VARCHAR NOT NULL,
    stage VARCHAR NOT NULL,
    PRIMARY KEY(game_id, requester_id)
);