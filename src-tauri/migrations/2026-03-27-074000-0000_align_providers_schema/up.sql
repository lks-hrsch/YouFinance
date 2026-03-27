-- Migrate providers table from (title, secret_id, secret_key) to (name, config_json)
PRAGMA foreign_keys = OFF;

CREATE TABLE providers_new (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at  TEXT
);

-- Migrate data: title -> name, build config_json from secret_id/secret_key
INSERT INTO providers_new (id, name, config_json, created_at, updated_at, deleted_at)
SELECT
    id,
    title,
    CASE
        WHEN title = 'GoCardless' AND secret_id IS NOT NULL
        THEN json_object('secret_id', secret_id, 'secret_key', secret_key)
        ELSE '{}'
    END,
    datetime('now'),
    datetime('now'),
    NULL
FROM providers;

DROP TABLE providers;
ALTER TABLE providers_new RENAME TO providers;

PRAGMA foreign_keys = ON;
