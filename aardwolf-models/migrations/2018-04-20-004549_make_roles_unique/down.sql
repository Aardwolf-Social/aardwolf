-- This file should undo anything in `up.sql`
ALTER TABLE roles DROP CONSTRAINT roles_unique_names;
