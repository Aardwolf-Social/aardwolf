-- Your SQL goes here
ALTER TABLE base_actors
    ADD COLUMN activitypub_id VARCHAR(300) UNIQUE NOT NULL;

ALTER TABLE base_posts
    ADD COLUMN activitypub_id VARCHAR(300) UNIQUE NOT NULL;
