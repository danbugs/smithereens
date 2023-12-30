DROP TABLE IF EXISTS last_checked_player_id;

CREATE TABLE pidgtm_compile_times (
    id SERIAL PRIMARY KEY,
    time_in_seconds INT NOT NULL,
    calculation_timestamp TIMESTAMP
);

CREATE TABLE error_logs (
    id SERIAL PRIMARY KEY,
    error_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);
