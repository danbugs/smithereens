ALTER TABLE players
ADD COLUMN prefix VARCHAR,
ADD COLUMN gamer_tag VARCHAR,
ADD COLUMN name VARCHAR,
ADD COLUMN state VARCHAR,
ADD COLUMN country VARCHAR,
ADD COLUMN profile_picture VARCHAR,
ADD COLUMN twitch_username VARCHAR,
ADD COLUMN twitter_username VARCHAR,
ADD COLUMN gender_pronouns VARCHAR,
ADD COLUMN birthday VARCHAR,
ADD COLUMN bio VARCHAR,
ADD COLUMN rankings TEXT ARRAY;

UPDATE players SET prefix = SPLIT_PART(gamer_tag_with_prefix, ' | ', 1) WHERE SPLIT_PART(gamer_tag_with_prefix, ' | ', 2) != '';
UPDATE players SET gamer_tag = SPLIT_PART(gamer_tag_with_prefix, ' | ', 2) WHERE SPLIT_PART(gamer_tag_with_prefix, ' | ', 2) != '';
UPDATE players SET gamer_tag = SPLIT_PART(gamer_tag_with_prefix, ' | ', 1) WHERE SPLIT_PART(gamer_tag_with_prefix, ' | ', 2) = '';

ALTER TABLE players ALTER COLUMN gamer_tag SET NOT NULL;

ALTER TABLE players DROP COLUMN gamer_tag_with_prefix;