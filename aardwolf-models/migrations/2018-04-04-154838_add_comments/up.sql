-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    conversation INTEGER REFERENCES posts(id) ON DELETE CASCADE NOT NULL,
    parent INTEGER REFERENCES posts(id) ON DELETE CASCADE NOT NULL,
    post INTEGER REFERENCES posts(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
