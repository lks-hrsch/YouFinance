-- Your SQL goes here
CREATE TABLE accounts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    provider_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    institution_id TEXT,
    bank_connection_id TEXT,
    account_id TEXT,
    iban TEXT,

    FOREIGN KEY (provider_id) REFERENCES providers(id)
)