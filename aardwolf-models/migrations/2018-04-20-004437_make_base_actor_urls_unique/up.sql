-- Your SQL goes here
ALTER TABLE base_actors ADD CONSTRAINT base_actors_unique_profile_url UNIQUE (profile_url);
ALTER TABLE base_actors ADD CONSTRAINT base_actors_unique_inbox_url UNIQUE (inbox_url);
ALTER TABLE base_actors ADD CONSTRAINT base_actors_unique_outbox_url UNIQUE (outbox_url);
