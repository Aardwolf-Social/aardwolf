-- This file should undo anything in `up.sql`
ALTER TABLE followers DROP CONSTRAINT followers_unique_follower;
