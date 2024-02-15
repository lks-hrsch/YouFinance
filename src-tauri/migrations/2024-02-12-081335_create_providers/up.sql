-- Your SQL goes here
CREATE TABLE providers (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  secret_id TEXT,
  secret_key TEXT
)