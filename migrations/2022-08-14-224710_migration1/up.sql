CREATE TABLE sets(
    id SERIAL PRIMARY KEY,
    completed_at BIGINT NOT NULL,
    requester_id INTEGER NOT NULL,
    requester_tag_with_prefix VARCHAR NOT NULL,
    requester_score INTEGER NOT NULL,
    opponent_tag_with_prefix VARCHAR NOT NULL,
    opponent_score INTEGER NOT NULL,
    result_type INTEGER NOT NULL,
    event_at_tournament VARCHAR NOT NULL,
    is_event_online BOOLEAN NOT NULL
);