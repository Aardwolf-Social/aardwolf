-- Your SQL goes here
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    file_path VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
