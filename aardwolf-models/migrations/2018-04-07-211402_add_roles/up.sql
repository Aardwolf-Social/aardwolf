-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);

INSERT INTO roles (name, created_at) VALUES ('admin', 'now');
INSERT INTO roles (name, created_at) VALUES ('moderator', 'now');
INSERT INTO roles (name, created_at) VALUES ('verified', 'now');
