-- Revert finalize_schema_alignment migration
PRAGMA foreign_keys = OFF;

-- 1. Recreate accounts table (pre-migration structure)
CREATE TABLE accounts (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id       INTEGER NOT NULL,
    title             TEXT NOT NULL,
    institution_id    TEXT,
    bank_connection_id TEXT NOT NULL,
    account_id        TEXT,
    iban              TEXT,
    last_synced_at    TEXT,
    FOREIGN KEY (provider_id) REFERENCES providers(id)
);

-- 2. Recreate transactions with account_id and currency
CREATE TABLE transactions_old (
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    booking_text           TEXT    NOT NULL DEFAULT '',
    debtor_name            TEXT,
    debtor_iban            TEXT,
    debtor_bic             TEXT,
    creditor_name          TEXT,
    creditor_iban          TEXT,
    creditor_bic           TEXT,
    amount_minor           INTEGER NOT NULL,
    currency               TEXT    NOT NULL,
    booking_date           TEXT    NOT NULL,
    value_date             TEXT,
    balance_after_minor    INTEGER,
    mandate_reference      TEXT,
    remittance_information TEXT,
    account_id             INTEGER NOT NULL,
    created_at             TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at             TEXT    NOT NULL DEFAULT (datetime('now')),
    deleted_at             TEXT,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    UNIQUE (booking_date, amount_minor, debtor_iban, creditor_iban)
);

INSERT INTO transactions_old (id, booking_text, debtor_name, debtor_iban, debtor_bic, creditor_name, creditor_iban, creditor_bic, amount_minor, currency, booking_date, value_date, balance_after_minor, mandate_reference, remittance_information, account_id, created_at, updated_at, deleted_at)
SELECT id, booking_text, debtor_name, debtor_iban, debtor_bic, creditor_name, creditor_iban, creditor_bic, amount_minor, currency_code, booking_date, value_date, balance_after_minor, mandate_reference, remittance_information, bank_account_id, created_at, updated_at, deleted_at
FROM transactions;

DROP TABLE transactions;
ALTER TABLE transactions_old RENAME TO transactions;

CREATE INDEX idx_transactions_account_id    ON transactions(account_id);
CREATE INDEX idx_transactions_booking_date  ON transactions(booking_date);
CREATE INDEX idx_transactions_debtor_iban   ON transactions(debtor_iban);
CREATE INDEX idx_transactions_creditor_iban ON transactions(creditor_iban);

-- 3. Recreate tags without audit columns
CREATE TABLE tags_old (
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL
);

INSERT INTO tags_old (id, title)
SELECT id, name FROM tags;

DROP TABLE tags;
ALTER TABLE tags_old RENAME TO tags;

-- 4. Recreate transaction_tags without audit columns
CREATE TABLE transaction_tags_old (
    transaction_id INTEGER NOT NULL,
    tag_id         INTEGER NOT NULL,
    PRIMARY KEY (transaction_id, tag_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

INSERT INTO transaction_tags_old (transaction_id, tag_id)
SELECT transaction_id, tag_id FROM transaction_tags;

DROP TABLE transaction_tags;
ALTER TABLE transaction_tags_old RENAME TO transaction_tags;

-- 5. Recreate transaction_providers without deleted_at
CREATE TABLE transaction_providers_old (
    transaction_id INTEGER NOT NULL,
    provider_id    INTEGER NOT NULL,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (transaction_id, provider_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id),
    FOREIGN KEY (provider_id) REFERENCES providers(id)
);

INSERT INTO transaction_providers_old (transaction_id, provider_id, created_at)
SELECT transaction_id, provider_id, created_at FROM transaction_providers;

DROP TABLE transaction_providers;
ALTER TABLE transaction_providers_old RENAME TO transaction_providers;

PRAGMA foreign_keys = ON;
