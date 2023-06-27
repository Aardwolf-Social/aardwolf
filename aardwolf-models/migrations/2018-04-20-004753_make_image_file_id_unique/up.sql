-- Your SQL goes here
ALTER TABLE images ADD CONSTRAINT images_unique_file_ids UNIQUE (file_id);
