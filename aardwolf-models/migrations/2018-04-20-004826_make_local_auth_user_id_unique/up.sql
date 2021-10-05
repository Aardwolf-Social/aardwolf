-- Your SQL goes here
ALTER TABLE local_auth ADD CONSTRAINT local_auth_unique_user_ids UNIQUE (user_id);
