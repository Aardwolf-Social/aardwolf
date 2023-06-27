-- Your SQL goes here
CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    file_id INTEGER REFERENCES files(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
