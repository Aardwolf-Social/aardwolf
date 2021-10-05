-- Your SQL goes here
CREATE TABLE followers (
    id SERIAL PRIMARY KEY,
    follower INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    follows INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
