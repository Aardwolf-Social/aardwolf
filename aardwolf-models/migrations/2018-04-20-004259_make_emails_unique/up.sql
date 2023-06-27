-- Your SQL goes here
ALTER TABLE emails ADD CONSTRAINT emails_unique_email UNIQUE (email);
