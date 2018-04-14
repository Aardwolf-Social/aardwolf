-- Your SQL goes here
CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    base_actor_id INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
