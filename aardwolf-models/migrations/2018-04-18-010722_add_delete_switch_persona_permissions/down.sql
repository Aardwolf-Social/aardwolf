-- This file should undo anything in `up.sql`
DELETE FROM permissions WHERE name = 'switch-persona';
DELETE FROM permissions WHERE name = 'delete-persona';
