-- This file should undo anything in `up.sql`
ALTER TABLE base_posts
    DROP COLUMN activitypub_id;

ALTER TABLE base_actors
    DROP COLUMN activitypub_id;
