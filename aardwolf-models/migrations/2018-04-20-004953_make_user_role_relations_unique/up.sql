-- Your SQL goes here
ALTER TABLE user_roles ADD CONSTRAINT user_roles_unique_relations UNIQUE (user_id, role_id);
