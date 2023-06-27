-- Your SQL goes here
CREATE TABLE reactions (
    id SERIAL PRIMARY KEY,
    reaction_type VARCHAR(10) NOT NULL,
    comment_id INTEGER REFERENCES comments(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
