CREATE TABLE IF NOT EXISTS fedibook.accounts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        VARCHAR NOT NULL DEFAULT '',
    domain          VARCHAR,
    display_name    VARCHAR NOT NULL DEFAULT '',

    created_at      TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at      TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc')
);

CREATE UNIQUE INDEX IF NOT EXISTS id_idx ON fedibook.accounts (id);
CREATE UNIQUE INDEX IF NOT EXISTS username_domain_unique_idx ON fedibook.accounts (username, domain);
