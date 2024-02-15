-- Your SQL goes here
CREATE TABLE transactions (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  debitor_name TEXT NOT NULL,
  debitor_iban TEXT NOT NULL,
  creditor_name TEXT NOT NULL,
  creditor_iban TEXT NOT NULL,
  ammount REAL NOT NULL,
  currency TEXT NOT NULL,
  date TIMESTAMP NOT NULL,
  remittance_information TEXT,
  account_id INTEGER,

  FOREIGN KEY (account_id) REFERENCES accounts(id)
)
