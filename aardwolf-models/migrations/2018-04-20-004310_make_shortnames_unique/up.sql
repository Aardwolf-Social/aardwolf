-- Your SQL goes here
ALTER TABLE personas ADD CONSTRAINT personas_unique_shortnames UNIQUE (shortname);
ALTER TABLE personas ADD CONSTRAINT personas_unique_actors UNIQUE (base_actor);
