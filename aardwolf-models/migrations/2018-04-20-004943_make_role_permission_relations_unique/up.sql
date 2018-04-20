-- Your SQL goes here
ALTER TABLE role_permissions ADD CONSTRAINT role_permissions_unique_relations UNIQUE (role_id, permission_id);
