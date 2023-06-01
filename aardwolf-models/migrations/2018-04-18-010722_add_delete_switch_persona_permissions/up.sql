-- Your SQL goes here
INSERT INTO permissions (name, created_at) VALUES ('switch-persona', 'now');
INSERT INTO permissions (name, created_at) VALUES ('delete-persona', 'now');

INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'switch-persona'),
    'now'
);
INSERT INTO role_permissions (role_id, permission_id, created_at) VALUES (
    (SELECT id FROM roles WHERE name = 'verified'),
    (SELECT id FROM permissions WHERE name = 'delete-persona'),
    'now'
);
