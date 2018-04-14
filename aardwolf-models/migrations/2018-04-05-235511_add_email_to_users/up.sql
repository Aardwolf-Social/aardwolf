-- Your SQL goes here
ALTER TABLE users ADD COLUMN primary_email INTEGER REFERENCES emails(id) ON DELETE CASCADE;
