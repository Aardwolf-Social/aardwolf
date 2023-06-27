-- Your SQL goes here
ALTER TABLE permissions ADD CONSTRAINT permissions_unique_names UNIQUE (name);
