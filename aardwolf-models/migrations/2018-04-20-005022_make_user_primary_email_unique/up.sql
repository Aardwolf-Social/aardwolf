-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT users_unique_primary_emails UNIQUE (primary_email);
