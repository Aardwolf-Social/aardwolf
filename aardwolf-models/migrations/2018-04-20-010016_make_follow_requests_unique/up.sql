-- Your SQL goes here
ALTER TABLE follow_requests ADD CONSTRAINT follow_requests_unique_follow_requests UNIQUE (follower, requested_follow);
