-- This file should undo anything in `up.sql`
ALTER TABLE base_actors
    DROP COLUMN private_key_der,
    DROP COLUMN public_key_der;
