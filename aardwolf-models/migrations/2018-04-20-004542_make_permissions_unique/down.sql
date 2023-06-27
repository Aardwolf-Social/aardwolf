-- This file should undo anything in `up.sql`
ALTER TABLE permissions DROP CONSTRAINT permissions_unique_names;
