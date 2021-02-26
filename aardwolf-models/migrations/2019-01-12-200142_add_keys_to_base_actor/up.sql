-- Your SQL goes here
ALTER TABLE base_actors
    ADD COLUMN private_key_der BYTEA NOT NULL,
    ADD COLUMN public_key_der BYTEA NOT NULL;
