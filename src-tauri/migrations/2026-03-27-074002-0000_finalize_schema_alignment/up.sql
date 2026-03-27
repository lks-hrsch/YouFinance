-- Finalize schema alignment: rename fields, add audit columns, drop old tables
PRAGMA foreign_keys = OFF;

-- 1. Recreate transactions with bank_account_id and currency_code
CREATE TABLE transactions_new (
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    booking_text           TEXT    NOT NULL DEFAULT '',
    debtor_name            TEXT,
    debtor_iban            TEXT,
    debtor_bic             TEXT,
    creditor_name          TEXT,
    creditor_iban          TEXT,
    creditor_bic           TEXT,
    amount_minor           INTEGER NOT NULL,
    currency_code          TEXT    NOT NULL,
    booking_date           TEXT    NOT NULL,
    value_date             TEXT,
    balance_after_minor    INTEGER,
    mandate_reference      TEXT,
    remittance_information TEXT,
    bank_account_id        INTEGER NOT NULL,
    created_at             TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at             TEXT    NOT NULL DEFAULT (datetime('now')),
    deleted_at             TEXT,
    FOREIGN KEY (bank_account_id) REFERENCES bank_accounts(id),
    UNIQUE (booking_date, amount_minor, debtor_iban, creditor_iban)
);

INSERT INTO transactions_new (id, booking_text, debtor_name, debtor_iban, debtor_bic, creditor_name, creditor_iban, creditor_bic, amount_minor, currency_code, booking_date, value_date, balance_after_minor, mandate_reference, remittance_information, bank_account_id, created_at, updated_at, deleted_at)
SELECT id, booking_text, debtor_name, debtor_iban, debtor_bic, creditor_name, creditor_iban, creditor_bic, amount_minor, currency, booking_date, value_date, balance_after_minor, mandate_reference, remittance_information, account_id, created_at, updated_at, deleted_at
FROM transactions;

DROP TABLE transactions;
ALTER TABLE transactions_new RENAME TO transactions;

CREATE INDEX idx_transactions_bank_account_id    ON transactions(bank_account_id);
CREATE INDEX idx_transactions_booking_date       ON transactions(booking_date);
CREATE INDEX idx_transactions_debtor_iban        ON transactions(debtor_iban);
CREATE INDEX idx_transactions_creditor_iban      ON transactions(creditor_iban);

-- 2. Recreate tags with audit columns
CREATE TABLE tags_new (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

INSERT INTO tags_new (id, name, created_at, updated_at, deleted_at)
SELECT id, title, datetime('now'), datetime('now'), NULL
FROM tags;

DROP TABLE tags;
ALTER TABLE tags_new RENAME TO tags;

-- 3. Add audit columns to transaction_tags
CREATE TABLE transaction_tags_new (
    transaction_id INTEGER NOT NULL,
    tag_id         INTEGER NOT NULL,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at     TEXT,
    PRIMARY KEY (transaction_id, tag_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

INSERT INTO transaction_tags_new (transaction_id, tag_id, created_at, deleted_at)
SELECT transaction_id, tag_id, datetime('now'), NULL
FROM transaction_tags;

DROP TABLE transaction_tags;
ALTER TABLE transaction_tags_new RENAME TO transaction_tags;

-- 4. Add deleted_at to transaction_providers (already has created_at)
CREATE TABLE transaction_providers_new (
    transaction_id INTEGER NOT NULL,
    provider_id    INTEGER NOT NULL,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at     TEXT,
    PRIMARY KEY (transaction_id, provider_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id),
    FOREIGN KEY (provider_id) REFERENCES providers(id)
);

INSERT INTO transaction_providers_new (transaction_id, provider_id, created_at, deleted_at)
SELECT transaction_id, provider_id, created_at, NULL
FROM transaction_providers;

DROP TABLE transaction_providers;
ALTER TABLE transaction_providers_new RENAME TO transaction_providers;

-- 5. Drop the old accounts table
DROP TABLE accounts;

PRAGMA foreign_keys = ON;
