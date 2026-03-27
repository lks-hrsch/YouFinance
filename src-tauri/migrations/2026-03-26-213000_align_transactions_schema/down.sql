-- Revert to old transactions schema
-- This converts amount_minor back to amount as float

-- Create old table schema
CREATE TABLE transactions_old (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    debitor_name TEXT,
    debitor_iban TEXT,
    debitor_bic TEXT,
    creditor_name TEXT,
    creditor_iban TEXT,
    creditor_bic TEXT,
    amount REAL NOT NULL,
    currency TEXT NOT NULL,
    date TEXT NOT NULL,
    remittance_information TEXT,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

-- Copy data back, converting amount_minor back to float
INSERT INTO transactions_old (
    id,
    title,
    debitor_name,
    debitor_iban,
    debitor_bic,
    creditor_name,
    creditor_iban,
    creditor_bic,
    amount,
    currency,
    date,
    remittance_information,
    account_id
)
SELECT
    id,
    booking_text,
    debtor_name,
    debtor_iban,
    debtor_bic,
    creditor_name,
    creditor_iban,
    creditor_bic,
    CAST(amount_minor AS REAL) / 100.0,
    currency,
    booking_date,
    remittance_information,
    account_id
FROM transactions;

-- Drop new table
DROP TABLE transactions;

-- Rename old table back
ALTER TABLE transactions_old RENAME TO transactions;

-- Recreate indexes
CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_date ON transactions(date);
CREATE INDEX idx_transactions_debitor_iban ON transactions(debitor_iban);
CREATE INDEX idx_transactions_creditor_iban ON transactions(creditor_iban);
