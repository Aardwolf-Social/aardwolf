-- Your SQL goes here
ALTER TABLE posts ADD CONSTRAINT posts_unique_base_posts UNIQUE (base_post);
