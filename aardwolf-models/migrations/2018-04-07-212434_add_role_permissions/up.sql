-- Your SQL goes here
CREATE TABLE role_permissions (
    id SERIAL PRIMARY KEY,
    role_id INTEGER REFERENCES roles(id) ON DELETE CASCADE NOT NULL,
    permission_id INTEGER REFERENCES permissions(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);

INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'follow-user'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'make-post'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'make-media-post'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'make-comment'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'make-persona'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'manage-follow-requests'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'admin'),
    (SELECT id FROM permissions WHERE name = 'configure-instance'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'admin'),
    (SELECT id FROM permissions WHERE name = 'grant-role'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'admin'),
    (SELECT id FROM permissions WHERE name = 'revoke-role'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'moderator'),
    (SELECT id FROM permissions WHERE name = 'ban-user'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'moderator'),
    (SELECT id FROM permissions WHERE name = 'block-instance'),
    'now'
);
