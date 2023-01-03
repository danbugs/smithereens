UPDATE player_games SET requester_win = false WHERE requester_win IS NULL;

ALTER TABLE player_games ALTER COLUMN requester_win SET NOT NULL;