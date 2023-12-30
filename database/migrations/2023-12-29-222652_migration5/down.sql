-- Commands to drop the new table for error logs
DROP TABLE IF EXISTS error_logs;

-- Commands to revert the repurposed table to its original form
ALTER TABLE pidgtm_compile_times DROP COLUMN calculation_timestamp;
ALTER TABLE pidgtm_compile_times ALTER COLUMN time_in_seconds SET DATA TYPE INTEGER;
ALTER TABLE pidgtm_compile_times RENAME COLUMN time_in_seconds TO player_id;
ALTER TABLE pidgtm_compile_times RENAME TO last_checked_player_id;
