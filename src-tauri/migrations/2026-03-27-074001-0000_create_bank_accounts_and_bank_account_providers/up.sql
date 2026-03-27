-- Create bank_accounts and bank_account_providers tables, migrate from accounts
PRAGMA foreign_keys = OFF;

-- Create bank_accounts table (account info only)
CREATE TABLE bank_accounts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL,
    iban            TEXT,
    bic             TEXT,
    owner_name      TEXT,
    currency_code   TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at      TEXT
);

-- Create bank_account_providers table (provider-account relationship)
CREATE TABLE bank_account_providers (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    bank_account_id     INTEGER NOT NULL,
    provider_id         INTEGER NOT NULL,
    bank_connection_id  TEXT NOT NULL,
    institution_id      TEXT,
    external_account_id TEXT,
    last_synced_at      TEXT,
    created_at          TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at          TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at          TEXT,
    FOREIGN KEY (bank_account_id) REFERENCES bank_accounts(id),
    FOREIGN KEY (provider_id) REFERENCES providers(id)
);

-- Migrate data from accounts to bank_accounts
INSERT INTO bank_accounts (id, name, iban, created_at, updated_at, deleted_at)
SELECT id, title, iban, datetime('now'), datetime('now'), NULL
FROM accounts;

-- Migrate provider relationships to bank_account_providers
INSERT INTO bank_account_providers (bank_account_id, provider_id, bank_connection_id, institution_id, external_account_id, last_synced_at, created_at, updated_at, deleted_at)
SELECT id, provider_id, bank_connection_id, institution_id, account_id, last_synced_at, datetime('now'), datetime('now'), NULL
FROM accounts;

-- Create index for lookups
CREATE INDEX idx_bank_account_providers_bank_account_id ON bank_account_providers(bank_account_id);
CREATE INDEX idx_bank_account_providers_provider_id ON bank_account_providers(provider_id);
CREATE INDEX idx_bank_account_providers_bank_connection_id ON bank_account_providers(bank_connection_id);

-- Note: Do NOT drop accounts table yet — transactions.account_id still references it.
-- It will be dropped in the finalize_schema_alignment migration.

PRAGMA foreign_keys = ON;
