-- Revert: restore transactions table to original state
PRAGMA foreign_keys = OFF;

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

INSERT INTO transactions_new
SELECT * FROM transactions;

DROP TABLE transactions;
ALTER TABLE transactions_new RENAME TO transactions;

CREATE INDEX idx_transactions_bank_account_id    ON transactions(bank_account_id);
CREATE INDEX idx_transactions_booking_date       ON transactions(booking_date);
CREATE INDEX idx_transactions_debtor_iban        ON transactions(debtor_iban);
CREATE INDEX idx_transactions_creditor_iban      ON transactions(creditor_iban);

PRAGMA foreign_keys = ON;
