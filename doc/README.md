# YouFinance Feature Specifications

Welcome to the YouFinance documentation directory. This folder contains structured markdown specifications that outline requirements, database schemas, and external data parsing patterns for key backend features in the application.

## 📖 Feature Index

- **[Loading CSV Data into Database](./features/load-csv-data-into-database.md)**
  - Outlines the structural requirements and data mapping necessary to deserialize standard comma-separated German banking exports into normalized backend structures. Highlights expected standard columns and empty field coercion.

- **[Loading MTA (MT940) Data into Database](./features/load-mta-data-into-database.md)**
  - A highly detailed specification for parsing standard `.mta` / `MT940` formats. Includes structured details detailing parser mappings for (`:61:`, `:86:`) markers, block-level configurations, and subfield (`?2x`) continuity rules. Addresses critical structural variations such as multiple-transaction blocks and fragmented remittance concatenation logic.

- **[Loading Live Data via GoCardless into Database](./features/load-live-data-via-gocardless-into-database.md)**
  - Documents the GoCardless Bank Account Data API integration: two-phase bank connection flow (requisitions), authentication, transaction sync commands, field mapping, and deduplication strategy.

## 📝 Documenting New Features

When expanding YouFinance structures, please create a dedicated markdown document within the `features/` subdirectory and formally link it here to maintain an accessible, high-level project map.

## Database Schema

Providers.config_json is a JSON object that contains the configuration for the provider.

For LocalCSV provider, the config_json is:

```json
{
    "data_dir": "/path/to/data/dir"
}
```

For GoCardless provider, the config_json is:

```json
{
    "secret_id": "secret_id",
    "secret_key": "secret_key"
}
```

``` mermaid
erDiagram
    PROVIDERS ||--o{ BANK_ACCOUNT_PROVIDERS : connected_to
    BANK_ACCOUNTS ||--o{ BANK_ACCOUNT_PROVIDERS : linked_via
    BANK_ACCOUNTS ||--o{ TRANSACTIONS : owns

    TRANSACTIONS ||--o{ TRANSACTION_TAGS : tagged
    TAGS ||--o{ TRANSACTION_TAGS : classifies

    TRANSACTIONS ||--o{ TRANSACTION_PROVIDERS : sourced_from
    PROVIDERS ||--o{ TRANSACTION_PROVIDERS : imported_via

    PROVIDERS {
        int id PK
        string name
        string config_json
        string created_at
        string updated_at
        string deleted_at
    }

    BANK_ACCOUNTS {
        int id PK
        string name
        string iban
        string bic
        string owner_name
        string currency_code
        string created_at
        string updated_at
        string deleted_at
    }

    BANK_ACCOUNT_PROVIDERS {
        int id PK
        int bank_account_id FK
        int provider_id FK
        string bank_connection_id
        string institution_id
        string external_account_id
        string last_synced_at
        string created_at
        string updated_at
        string deleted_at
    }

    TRANSACTIONS {
        int id PK
        int bank_account_id FK
        string booking_date
        string value_date
        string currency_code
        string booking_text
        string remittance_information
        string debtor_name
        string debtor_iban
        string debtor_bic
        string creditor_name
        string creditor_iban
        string creditor_bic
        string mandate_reference
        int amount_minor
        int balance_after_minor
        string created_at
        string updated_at
        string deleted_at
    }

    TAGS {
        int id PK
        string name
        string created_at
        string updated_at
        string deleted_at
    }

    TRANSACTION_TAGS {
        int transaction_id PK, FK
        int tag_id PK, FK
        string created_at
        string deleted_at
    }

    TRANSACTION_PROVIDERS {
        int transaction_id PK, FK
        int provider_id PK, FK
        string created_at
        string deleted_at
    }
```

### TRANSACTION_PROVIDERS Table

The `TRANSACTION_PROVIDERS` table links each imported transaction to the provider that sourced it. This enables tracking of transaction lineage and supports reconciliation workflows where transactions from different providers (e.g., CSV imports, GoCardless API, MTA files) may be cross-referenced. When a transaction is imported from any provider, a corresponding row must be inserted into `TRANSACTION_PROVIDERS` with the transaction ID and provider ID.

### Schema Constraints and Notes

- **UNIQUE Constraint**: The `TRANSACTIONS` table has a UNIQUE constraint on (bank_account_id, booking_date, amount_minor, currency_code, debtor_iban, creditor_iban) to prevent duplicate transactions from being imported multiple times.
- **Soft Deletes**: The `deleted_at` field is present in all tables but is currently scaffolded and not yet activated. In the future, this will enable soft deletion (marking records as deleted without removing them) for audit and recovery purposes.
- **Amount Storage**: Monetary amounts are stored as `int` (32-bit integer) in minor units (e.g., cents for EUR). This supports values up to ±21.4 million EUR, which is appropriate for personal finance applications.
