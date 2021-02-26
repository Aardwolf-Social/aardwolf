-- Your SQL goes here
CREATE TABLE timers (
    id SERIAL PRIMARY KEY,
    fire_time TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
