-- This file should undo anything in `up.sql`
ALTER TABLE personas DROP CONSTRAINT personas_unique_shortnames;
ALTER TABLE personas DROP CONSTRAINT personas_unique_actors;
