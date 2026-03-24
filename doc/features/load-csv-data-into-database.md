# load csv data into the database

The system shall support importing transaction data from CSV files into the database.

CSV files are provided in a folder structure organized by provider and year.

## Input File Structure

CSV files shall be stored in the following directory structure:

```text
<data_folder>/
└── <provider_documents>/
    └── <year>/
        └── <document_name>.csv
```

### Example

```text
data/
└── bank_provider/
    └── 2024/
        └── account_transactions.csv
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

```rust
diesel::table! {
    transactions (id) {
        id -> Integer,
        title -> Text,
        debitor_name -> Nullable<Text>,
        debitor_iban -> Nullable<Text>,
        debitor_bic -> Nullable<Text>,
        creditor_name -> Nullable<Text>,
        creditor_iban -> Nullable<Text>,
        creditor_bic -> Nullable<Text>,
        amount -> Double,
        currency -> Text,
        date -> Text,
        remittance_information -> Nullable<Text>,
        account_id -> Integer,
    }
}
```

## Field Mapping

The following CSV columns shall be mapped to the database fields:

| CSV Column | Database Column |
|------------|-----------------|
| Bezeichnung Auftragskonto | title |
| Bankname Auftragskonto | debitor_name |
| IBAN Auftragskonto | debitor_iban |
| Name Zahlungsbeteiligter | creditor_name |
| IBAN Zahlungsbeteiligter | creditor_iban |
| Betrag | amount |
| Waehrung | currency |
| Buchungstag | date |
| Verwendungszweck | remittance_information |

## Notes on Unmapped CSV Columns

The CSV contains additional columns that are currently not represented in the target table, including:

- `Valutadatum`
- `Buchungstext`
- `Saldo nach Buchung`
- `Bemerkung`
- `Gekennzeichneter Umsatz`
- `Glaeubiger ID`
- `Mandatsreferenz`

These fields shall be ignored unless the schema is extended.

## Data Conversion Rules

The importer shall apply the following transformations:

### Date
- Source format: `DD.MM.YYYY`
- Target format: keep as text unless a different database format is required.
- Example: `30.12.2024`

### Amount
- Source format uses a comma as decimal separator, e.g. `-17,49`
- The value shall be converted to a numeric `Double`
- Example:
  - CSV: `-17,49`
  - Database: `-17.49`

### Empty Values
- Empty CSV fields shall be stored as `NULL` for nullable database columns.

## Functional Requirements

1. The system shall recursively scan the configured data folder for CSV files matching the expected directory structure.
2. The system shall parse CSV files using semicolon (`;`) as delimiter.
3. The system shall validate that the CSV header matches the expected format.
4. The system shall transform and import each row into the `transactions` table.
5. The system shall convert the `Betrag` field from German decimal notation to a `Double`.
6. The system shall store empty optional values as `NULL`.
7. The system shall log or report rows that cannot be imported due to invalid format or missing required values.
8. The system should continue processing remaining rows if a single row fails, unless configured otherwise.
