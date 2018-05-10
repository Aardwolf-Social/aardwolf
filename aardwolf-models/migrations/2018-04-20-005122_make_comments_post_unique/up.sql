-- Your SQL goes here
ALTER TABLE comments ADD CONSTRAINT comments_unique_posts UNIQUE (post);
