-- Your SQL goes here
CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    href VARCHAR(2048) NOT NULL,
    href_lang VARCHAR(8) NOT NULL,
    height INTEGER,
    width INTEGER,
    preview TEXT,
    base_post INTEGER REFERENCES base_posts(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() at time zone 'utc')
);
