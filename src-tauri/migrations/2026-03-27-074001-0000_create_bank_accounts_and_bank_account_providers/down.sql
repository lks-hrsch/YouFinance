-- Drop bank_accounts and bank_account_providers tables
PRAGMA foreign_keys = OFF;

DROP TABLE IF EXISTS bank_account_providers;
DROP TABLE IF EXISTS bank_accounts;

PRAGMA foreign_keys = ON;
