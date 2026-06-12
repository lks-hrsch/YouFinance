# load csv data into the database

The system shall support importing transaction data from CSV files into the database.

CSV files are provided in a folder structure organized by provider and year.

## Input File Structure

CSV files shall be stored in the following directory structure:

```text
<data_folder>/
└── <provider_documents>/
    └── <year>_<document_name>.csv
```

### Example

```text
data/
└── bank_provider/
    └── 2024_account_transactions.csv
```

## CSV Format

Each CSV file shall use the following header row:

```text
Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Gekennzeichneter Umsatz;Glaeubiger ID;Mandatsreferenz
```

The delimiter is a semicolon (`;`).

## Example CSV Row (Anonymized)

```text
BusinessAccount;DE00XXXX00000000000000;BANKDEFFXXX;Sample Bank;30.12.2024;30.12.2024;Telecom Provider GmbH;DE00XXXX00000000000001;HYVEDEMMXXX;LASTSCHRIFT;Customer No.: XXXXXXXX, Invoice No.: XXXXXXXXXX, Monthly service charge EREF: XXXXXXXXXXXXXXXXXXXXXXXXXXXX MREF: XXXXXXXXXXXXXXXXXXXXXXXXXXXX CRED: DE97XXXXXXXXXXXXXXX IBAN: DE00XXXX00000000000001 BIC: HYVEDEMMXXX;-17,49;EUR;1853,02;;;DE97XXXXXXXXXXXXXXX;XXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

## Target Database Table

The imported data shall be inserted into the `transactions` table:

```
TRANSACTIONS {
    id               PK
    bank_account_id  FK -> BANK_ACCOUNTS.id
    booking_date     text
    value_date       text (nullable)
    currency_code    text
    booking_text     text (nullable)
    remittance_information text (nullable)
    debtor_name      text (nullable)
    debtor_iban      text (nullable)
    debtor_bic       text (nullable)
    creditor_name    text (nullable)
    creditor_iban    text (nullable)
    creditor_bic     text (nullable)
    mandate_reference text (nullable)
    amount_minor     integer
    balance_after_minor integer (nullable)
    created_at       text
    updated_at       text
    deleted_at       text (nullable)
}
```

## Field Mapping

The following CSV columns shall be mapped to the database fields:

| CSV Column | Database Column | Notes |
|------------|-----------------|-------|
| Buchungstag | booking_date | converted to `YYYY-MM-DD` |
| Valutadatum | value_date | converted to `YYYY-MM-DD` |
| Buchungstext | booking_text | |
| Verwendungszweck | remittance_information | |
| Bezeichnung Auftragskonto | debtor_name | |
| IBAN Auftragskonto | debtor_iban | |
| BIC Auftragskonto | debtor_bic | |
| Name Zahlungsbeteiligter | creditor_name | |
| IBAN Zahlungsbeteiligter | creditor_iban | |
| BIC (SWIFT-Code) Zahlungsbeteiligter | creditor_bic | |
| Mandatsreferenz | mandate_reference | |
| Betrag | amount_minor | converted from German decimal to integer minor units (e.g. cents) |
| Waehrung | currency_code | |
| Saldo nach Buchung | balance_after_minor | converted from German decimal to integer minor units |
| *(account context)* | bank_account_id | resolved from the linked `bank_accounts` record |

### Still unmapped

The following CSV columns are still not represented in the target table:

- `Bemerkung`
- `Gekennzeichneter Umsatz`
- `Glaeubiger ID`

These fields shall be ignored unless the schema is extended.

## Data Conversion Rules

The importer shall apply the following transformations:

### Date

- Source format: `DD.MM.YYYY`
- Target format: `YYYY-MM-DD`
- Example:
  - CSV: `30.12.2024`
  - Database: `2024-12-30`

### Amount

- Source format uses a comma as decimal separator, e.g. `-17,49`
- The value shall be converted to an integer in minor currency units (e.g. cents)
- Example:
  - CSV: `-17,49`
  - Database: `-1749`

### Balance

- Same rules as Amount apply to `Saldo nach Buchung` → `balance_after_minor`.

### Empty Values

- Empty CSV fields shall be stored as `NULL` for nullable database columns.

## Provider Tracking

Each imported transaction must also insert a row into the `TRANSACTION_PROVIDERS` table linking the transaction to the LocalCSV provider. This allows tracking of which provider sourced each transaction and enables reconciliation workflows where transactions from multiple sources may need to be cross-referenced.

## Functional Requirements

1. The system shall recursively scan the configured data folder for CSV files matching the expected directory structure.
2. The system shall parse CSV files using semicolon (`;`) as delimiter.
3. The system shall validate that the CSV header matches the expected format.
4. The system shall transform and import each row into the `transactions` table.
5. The system shall convert the `Betrag` field from German decimal notation to an integer in minor currency units.
6. The system shall convert the `Saldo nach Buchung` field from German decimal notation to an integer in minor currency units.
7. The system shall convert dates from `DD.MM.YYYY` to `YYYY-MM-DD`.
8. The system shall store empty optional values as `NULL`.
9. The system shall log or report rows that cannot be imported due to invalid format or missing required values.
10. The system should continue processing remaining rows if a single row fails, unless configured otherwise.
11. The system shall insert a corresponding row into `TRANSACTION_PROVIDERS` for each imported transaction, linking it to the LocalCSV provider.
