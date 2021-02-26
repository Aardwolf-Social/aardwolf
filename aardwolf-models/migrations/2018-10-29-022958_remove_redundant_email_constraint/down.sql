-- This file should undo anything in `up.sql`
ALTER TABLE emails ADD CONSTRAINT emails_unique_email UNIQUE (email);
