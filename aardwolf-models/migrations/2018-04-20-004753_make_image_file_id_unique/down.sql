-- This file should undo anything in `up.sql`
ALTER TABLE images DROP CONSTRAINT images_unique_file_ids;
