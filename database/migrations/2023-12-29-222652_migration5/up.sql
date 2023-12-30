-- Commands to repurpose the existing table
ALTER TABLE last_checked_player_id RENAME TO pidgtm_compile_times;
ALTER TABLE pidgtm_compile_times RENAME COLUMN player_id TO time_in_seconds;
ALTER TABLE pidgtm_compile_times ALTER COLUMN time_in_seconds SET DATA TYPE INTEGER;
ALTER TABLE pidgtm_compile_times ADD COLUMN calculation_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- Commands to create a new table for error logs
CREATE TABLE error_logs (
    id SERIAL PRIMARY KEY,
    error_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);
