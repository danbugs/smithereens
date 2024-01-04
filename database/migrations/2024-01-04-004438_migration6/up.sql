CREATE TABLE player_page_views (
    id SERIAL PRIMARY KEY,
    access_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    player_id INTEGER NOT NULL
);
