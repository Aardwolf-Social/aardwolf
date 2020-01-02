-- Your SQL goes here
CREATE TABLE group_base_actors (
    id SERIAL PRIMARY KEY,
    group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE NOT NULL,
    base_actor_id INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
