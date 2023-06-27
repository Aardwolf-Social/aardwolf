-- This file should undo anything in `up.sql`
ALTER TABLE emails DROP CONSTRAINT emails_unique_email;
