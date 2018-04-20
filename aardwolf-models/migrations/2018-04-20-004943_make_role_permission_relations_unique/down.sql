-- This file should undo anything in `up.sql`
ALTER TABLE role_permissions DROP CONSTRAINT role_permissions_unique_relations;
