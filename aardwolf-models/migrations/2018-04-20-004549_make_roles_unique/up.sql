-- Your SQL goes here
ALTER TABLE roles ADD CONSTRAINT roles_unique_names UNIQUE (name);
