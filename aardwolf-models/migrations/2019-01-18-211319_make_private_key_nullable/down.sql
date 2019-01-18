-- This file should undo anything in `up.sql`
ALTER TABLE base_actors ALTER COLUMN private_key_der SET NOT NULL;
