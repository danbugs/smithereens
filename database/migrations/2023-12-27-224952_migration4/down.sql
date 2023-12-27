-- Revert primary key change in player_games
ALTER TABLE player_games
DROP CONSTRAINT player_games_pkey;

ALTER TABLE player_games
ADD PRIMARY KEY (game_id, requester_id);

-- Remove the set_id column from player_games
ALTER TABLE player_games
DROP COLUMN set_id;

-- Add the game_ids column back to player_sets
ALTER TABLE player_sets
ADD COLUMN game_ids INTEGER ARRAY;
