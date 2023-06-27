-- This file should undo anything in `up.sql`
ALTER TABLE local_auth DROP CONSTRAINT local_auth_unique_user_ids;
