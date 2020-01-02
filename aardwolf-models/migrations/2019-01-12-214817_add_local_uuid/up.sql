-- Your SQL goes here
ALTER TABLE base_actors
    ADD COLUMN local_uuid UUID;

ALTER TABLE base_posts
    ADD COLUMN local_uuid UUID;
