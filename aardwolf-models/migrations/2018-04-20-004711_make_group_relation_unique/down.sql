-- This file should undo anything in `up.sql`
ALTER TABLE group_base_actors DROP CONSTRAINT group_base_actors_unique_group_relations;
