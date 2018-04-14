-- Your SQL goes here
CREATE TABLE base_posts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(140),
    media_type VARCHAR(120) NOT NULL,
    posted_by INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    icon INTEGER REFERENCES images(id) ON DELETE CASCADE,
    visibility VARCHAR(8) NOT NULL,
    original_json JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
