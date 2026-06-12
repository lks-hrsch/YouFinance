CREATE TABLE transaction_providers (
    transaction_id INTEGER NOT NULL REFERENCES transactions(id),
    provider_id    INTEGER NOT NULL REFERENCES providers(id),
    created_at     TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (transaction_id, provider_id)
);
