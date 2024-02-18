-- Your SQL goes here
CREATE TABLE transactions (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  debitor_name TEXT,
  debitor_iban TEXT,
  creditor_name TEXT,
  creditor_iban TEXT,
  amount DOUBLE NOT NULL,
  currency TEXT NOT NULL,
  date TEXT NOT NULL,
  remittance_information TEXT,
  account_id INTEGER NOT NULL,

  FOREIGN KEY (account_id) REFERENCES accounts(id)
)
