-- Drop the game_ids column from player_sets
ALTER TABLE player_sets
DROP COLUMN game_ids;

-- Add the set_id column to player_games
ALTER TABLE player_games
ADD COLUMN set_id INTEGER;

-- Update the primary key of player_games
ALTER TABLE player_games
DROP CONSTRAINT player_games_pkey;

ALTER TABLE player_games
ADD PRIMARY KEY (game_id, requester_id, set_id);
