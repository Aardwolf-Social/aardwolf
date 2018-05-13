-- Your SQL goes here
ALTER TABLE followers ADD CONSTRAINT followers_unique_follower UNIQUE (follower, follows);
