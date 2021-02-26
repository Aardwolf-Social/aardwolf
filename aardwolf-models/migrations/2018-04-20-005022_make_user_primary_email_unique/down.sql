-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT users_unique_primary_emails;
