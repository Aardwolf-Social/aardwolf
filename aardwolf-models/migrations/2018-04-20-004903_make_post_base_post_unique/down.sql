-- This file should undo anything in `up.sql`
ALTER TABLE posts DROP CONSTRAINT posts_unique_base_posts;
