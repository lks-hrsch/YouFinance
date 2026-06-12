-- Align transactions table with documented schema
-- Add new fields and rename existing ones

-- Create new table with correct schema
CREATE TABLE transactions_new (
    id INTEGER PRIMARY KEY,
    booking_text TEXT NOT NULL,
    debtor_name TEXT,
    debtor_iban TEXT,
    debtor_bic TEXT,
    creditor_name TEXT,
    creditor_iban TEXT,
    creditor_bic TEXT,
    amount_minor INTEGER NOT NULL,
    currency TEXT NOT NULL,
    booking_date TEXT NOT NULL,
    value_date TEXT,
    balance_after_minor INTEGER,
    mandate_reference TEXT,
    remittance_information TEXT,
    account_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

-- Copy data from old table, converting amount to minor units
INSERT INTO transactions_new (
    id,
    booking_text,
    debtor_name,
    debtor_iban,
    debtor_bic,
    creditor_name,
    creditor_iban,
    creditor_bic,
    amount_minor,
    currency,
    booking_date,
    value_date,
    balance_after_minor,
    mandate_reference,
    remittance_information,
    account_id,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    id,
    title,
    debitor_name,
    debitor_iban,
    debitor_bic,
    creditor_name,
    creditor_iban,
    creditor_bic,
    CAST((amount * 100.0) AS INTEGER),
    currency,
    date,
    NULL,
    NULL,
    NULL,
    remittance_information,
    account_id,
    datetime('now'),
    datetime('now'),
    NULL
FROM transactions;

-- Drop old table
DROP TABLE transactions;

-- Rename new table
ALTER TABLE transactions_new RENAME TO transactions;

-- Recreate indexes if needed
CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_booking_date ON transactions(booking_date);
CREATE INDEX idx_transactions_debtor_iban ON transactions(debtor_iban);
CREATE INDEX idx_transactions_creditor_iban ON transactions(creditor_iban);
