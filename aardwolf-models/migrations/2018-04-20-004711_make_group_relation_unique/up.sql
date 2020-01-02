-- Your SQL goes here
ALTER TABLE group_base_actors ADD CONSTRAINT group_base_actors_unique_group_relations UNIQUE (group_id, base_actor_id);
