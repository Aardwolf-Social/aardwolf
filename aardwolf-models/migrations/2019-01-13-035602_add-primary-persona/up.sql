-- Your SQL goes here
ALTER TABLE users
    ADD COLUMN primary_persona INTEGER REFERENCES personas(id) ON DELETE CASCADE;
