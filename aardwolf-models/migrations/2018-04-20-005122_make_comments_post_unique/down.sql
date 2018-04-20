-- This file should undo anything in `up.sql`
ALTER TABLE comments DROP CONSTRAINT comments_unique_posts;
