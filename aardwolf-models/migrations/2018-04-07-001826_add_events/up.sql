-- Your SQL goes here
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    owner INTEGER REFERENCES personas(id) ON DELETE CASCADE NOT NULL,
    start_date INTEGER REFERENCES timers(id) ON DELETE CASCADE NOT NULL,
    end_date INTEGER REFERENCES timers(id) ON DELETE CASCADE NOT NULL,
    timezone VARCHAR(80) NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
