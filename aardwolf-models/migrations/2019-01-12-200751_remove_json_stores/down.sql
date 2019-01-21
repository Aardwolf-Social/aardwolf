-- This file should undo anything in `up.sql`
ALTER TABLE base_posts ADD COLUMN original_json JSONB;
ALTER TABLE base_actors ADD COLUMN original_json JSONB;
