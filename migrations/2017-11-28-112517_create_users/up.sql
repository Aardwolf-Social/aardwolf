CREATE TABLE IF NOT EXISTS aardwolf.users (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email                   VARCHAR NOT NULL DEFAULT '',
    encrypted_password      VARCHAR NOT NULL,
    account_id              UUID NOT NULL,

    -- flags
    admin                   BOOLEAN NOT NULL DEFAULT false,
    disabled                BOOLEAN NOT NULL DEFAULT false,

    -- confirmation stuff
    unconfirmed_email       VARCHAR NOT NULL DEFAULT '',
    confirmation_token      BYTEA NOT NULL,
    confirmed_at            TIMESTAMP WITHOUT TIME ZONE,
    confirmation_sent_at    TIMESTAMP WITHOUT TIME ZONE,

    -- timestamps
    created_at              TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc'),
    updated_at              TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc'),

    FOREIGN KEY (account_id) REFERENCES aardwolf.accounts (id)
);

CREATE INDEX IF NOT EXISTS email_idx ON aardwolf.users (email);
