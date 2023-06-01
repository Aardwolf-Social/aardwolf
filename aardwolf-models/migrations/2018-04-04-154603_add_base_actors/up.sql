-- Your SQL goes here
CREATE TABLE base_actors (
    id SERIAL PRIMARY KEY,
    display_name VARCHAR(80) NOT NULL,
    profile_url VARCHAR(2048) NOT NULL,
    inbox_url VARCHAR(2048) NOT NULL,
    outbox_url VARCHAR(2048) NOT NULL,
    local_user INTEGER REFERENCES users(id) ON DELETE CASCADE,
    follow_policy VARCHAR(8) NOT NULL,
    original_json JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
