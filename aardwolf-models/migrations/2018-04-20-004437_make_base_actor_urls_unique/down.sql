-- This file should undo anything in `up.sql`
ALTER TABLE base_actors DROP CONSTRAINT base_actors_unique_profile_url;
ALTER TABLE base_actors DROP CONSTRAINT base_actors_unique_inbox_url;
ALTER TABLE base_actors DROP CONSTRAINT base_actors_unique_outbox_url;
