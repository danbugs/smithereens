ALTER TABLE players ADD COLUMN gamer_tag_with_prefix VARCHAR;

UPDATE players SET gamer_tag_with_prefix = CONCAT_WS(' | ', prefix, gamer_tag);

ALTER TABLE players ALTER COLUMN gamer_tag_with_prefix SET NOT NULL;

ALTER TABLE players
DROP COLUMN prefix,
DROP COLUMN gamer_tag,
DROP COLUMN name,
DROP COLUMN state,
DROP COLUMN country,
DROP COLUMN profile_picture,
DROP COLUMN twitch_username,
DROP COLUMN twitter_username,
DROP COLUMN gender_pronouns,
DROP COLUMN birthday,
DROP COLUMN bio,
DROP COLUMN rankings;
