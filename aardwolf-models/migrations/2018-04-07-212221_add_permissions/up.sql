-- Your SQL goes here
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);

INSERT INTO permissions (name, created_at) VALUES ('follow-user', 'now');
INSERT INTO permissions (name, created_at) VALUES ('make-post', 'now');
INSERT INTO permissions (name, created_at) VALUES ('make-media-post', 'now');
INSERT INTO permissions (name, created_at) VALUES ('make-comment', 'now');
INSERT INTO permissions (name, created_at) VALUES ('configure-instance', 'now');
INSERT INTO permissions (name, created_at) VALUES ('ban-user', 'now');
INSERT INTO permissions (name, created_at) VALUES ('block-instance', 'now');
INSERT INTO permissions (name, created_at) VALUES ('grant-role', 'now');
INSERT INTO permissions (name, created_at) VALUES ('revoke-role', 'now');
INSERT INTO permissions (name, created_at) VALUES ('make-persona', 'now');
INSERT INTO permissions (name, created_at) VALUES ('manage-follow-requests', 'now');
