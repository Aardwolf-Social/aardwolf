-- Your SQL goes here
CREATE TABLE personas (
    id SERIAL PRIMARY KEY,
    default_visibility VARCHAR(8) NOT NULL,
    is_searchable BOOLEAN NOT NULL,
    avatar INTEGER REFERENCES images(id) ON DELETE CASCADE,
    shortname VARCHAR(80) NOT NULL,
    base_actor INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
