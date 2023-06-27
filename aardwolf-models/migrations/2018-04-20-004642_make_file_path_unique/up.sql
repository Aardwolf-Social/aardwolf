-- Your SQL goes here
ALTER TABLE files ADD CONSTRAINT files_unique_file_path UNIQUE (file_path);
