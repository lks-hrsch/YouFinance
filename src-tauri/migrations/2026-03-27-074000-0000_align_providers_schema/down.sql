-- Revert: restore providers table with (title, secret_id, secret_key)
PRAGMA foreign_keys = OFF;

CREATE TABLE providers_old (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    title     TEXT NOT NULL,
    secret_id TEXT,
    secret_key TEXT
);

-- Migrate back: name -> title, extract secret_id/secret_key from config_json
INSERT INTO providers_old (id, title, secret_id, secret_key)
SELECT
    id,
    name,
    CASE WHEN config_json != '{}' AND json_extract(config_json, '$.secret_id') IS NOT NULL
         THEN json_extract(config_json, '$.secret_id')
         ELSE NULL
    END,
    CASE WHEN config_json != '{}' AND json_extract(config_json, '$.secret_key') IS NOT NULL
         THEN json_extract(config_json, '$.secret_key')
         ELSE NULL
    END
FROM providers;

DROP TABLE providers;
ALTER TABLE providers_old RENAME TO providers;

PRAGMA foreign_keys = ON;
