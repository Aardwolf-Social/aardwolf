-- This file should undo anything in `up.sql`
ALTER TABLE files DROP CONSTRAINT files_unique_file_path;
