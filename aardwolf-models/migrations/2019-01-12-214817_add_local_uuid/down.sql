-- This file should undo anything in `up.sql`
ALTER TABLE base_posts
    DROP COLUMN local_uuid;

ALTER TABLE base_actors
    DROP COLUMN local_uuid;
