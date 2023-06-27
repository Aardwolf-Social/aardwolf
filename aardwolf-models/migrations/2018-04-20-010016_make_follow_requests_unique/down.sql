-- This file should undo anything in `up.sql`
ALTER TABLE follow_requests DROP CONSTRAINT follow_requests_unique_follow_requests;
