# load mta data into the database

The system shall support importing transaction data from MTA files (MT940 Standard) into the database.  
MTA files are provided in a folder structure organized by provider and year.

## Input File Structure

MTA files shall be stored in the following directory structure:

```text
<data_folder>/
└── <provider_documents>/
    └── <year>/
        └── <document_name>.mta
```

### Example

```text
data/
└── bank_provider/
    └── 2024/
        └── account_transactions.mta
```

## MTA Format

Each MTA file consists of one or more transaction blocks (bank statements) separated by a dash `-`.  
A single statement block contains global account and balance information for the period, but may contain *multiple* sequential `:61:` (transaction line) and `:86:` (transaction details) tag pairs. The parser must dynamically iterate over these pairs as a continuous stream rather than assuming a strict 1:1 block-to-transaction ratio.

The format uses tagged lines such as:

- `:20:` statement reference
- `:25:` account identification
- `:28C:` statement number
- `:60F:` opening balance
- `:61:` transaction line
- `:86:` transaction details
- `:62F:` closing balance

A transaction block is separated by a single line containing:

```text
-
```

## Example MTA Transaction Block (Anonymized)

```text
-
:20:STARTUMS
:25:00000000/0000000000
:28C:0
:60F:C240110EUR1367,23
:61:2401100110DR46,55NDDTKREF+
:86:105?00EINZUGSERMAECHTIGUNG?20EREF+XXXXXXXXXXXX
?21KREF+XXXXXXXXXXXXXXXXXXXX?22MREF+XXXXXXXXXXXX
?23CRED+DE00ZZZ00000000000?24SVWZ+Monthly subscription
?30BANKDEFFXXX?31DE00XXXXXXXXXXXXXXX?32Example Payee GmbH
:62F:C240110EUR1320,68
```

## Target Database Table

The imported data shall be inserted into the `transactions` table:

```
TRANSACTIONS {
    id                   PK
    bank_account_id      FK -> BANK_ACCOUNTS.id
    booking_date         text
    value_date           text (nullable)
    currency_code        text
    booking_text         text (nullable)
    remittance_information text (nullable)
    debtor_name          text (nullable)
    debtor_iban          text (nullable)
    debtor_bic           text (nullable)
    creditor_name        text (nullable)
    creditor_iban        text (nullable)
    creditor_bic         text (nullable)
    mandate_reference    text (nullable)
    amount_minor         integer
    balance_after_minor  integer (nullable)
    created_at           text
    updated_at           text
    deleted_at           text (nullable)
}
```

## MTA Field Extraction

The importer shall extract relevant transaction data from the tagged MTA content.

### Relevant Tags

| Tag | Meaning |
|-----|---------|
| `:25:` | Account number / source account |
| `:61:` | Booking date, value date, debit/credit indicator, amount |
| `:86:` | Transaction details including type, remittance information, counterpart account, counterpart name |
| `:60F:` | Opening balance |
| `:62F:` | Closing balance |

## Field Mapping

The following MTA values shall be mapped to the database fields:

| MTA Source | Database Column |
|------------|-----------------|
| `:61:` booking date (`YYMMDD`) | `booking_date` |
| `:61:` value date (`MMDD` or `YYMMDD`) | `value_date` |
| `:61:` amount | `amount_minor` |
| `:61:` currency inferred from (`:60F:` / `:62F:`) | `currency_code` |
| `:86:?00...` transaction type text | `booking_text` |
| `:86:?32...` counterpart name | `creditor_name` |
| `:86:?31...` counterpart IBAN | `creditor_iban` |
| `:86:?30...` counterpart BIC | `creditor_bic` |
| `:86:?2x...SVWZ+...` remittance information | `remittance_information` |
| *(account context)* | `bank_account_id` | resolved from the linked `bank_accounts` record |

## Interpretation Rules

Because MTA files do not provide the fields in exactly the same shape as the CSV import, the importer shall interpret the data as follows.

### booking_text

The `booking_text` field shall be filled from the transaction type in the `:86:` segment.

Examples:
- `EINZUGSERMAECHTIGUNG`
- `GUTSCHRIFT`
- `SEPA-Ueberweisung`

This is usually encoded in `?00`.

### booking_date and value_date

The `booking_date` field shall be taken from the `:61:` tag (first 6 digits: `YYMMDD`).

The `value_date` field shall be taken from the next 4 digits in `:61:` (`MMDD`, year inferred from statement context).

Example:

```text
:61:2401100110DR46,55NDDTKREF+
```

- Booking date: `240110` → `2024-01-10`
- Value date: `0110` → `2024-01-10`

The importer shall convert dates to the format:

```text
YYYY-MM-DD
```

### amount_minor

The `amount_minor` field shall be taken from the `:61:` tag and stored as an integer in minor currency units (e.g. cents).

Examples:
- `DR46,55` means `-4655`
- `CR28,00` means `2800`

Rules:
- `DR` indicates a debit and shall be stored as a negative integer
- `CR` indicates a credit and shall be stored as a positive integer
- the comma decimal separator shall be removed and the value scaled to minor units

### currency_code

The `currency_code` field shall be determined from the statement currency in `:60F:` or `:62F:`.

Example:

```text
:60F:C240110EUR1367,23
```

Currency = `EUR`

### remittance_information

The `remittance_information` field shall be extracted from the `:86:` subfields beginning with `SVWZ+`.

Because bank exports frequently exceed the character limit of a single `?2x` tag, the structured remittance text may arbitrarily overflow into immediate subsequent `?2x` or `?6x` tags (like `?24`, `?25`, up to `?29`) without restating the `SVWZ+` marker. The parser must intelligently capture the initial `SVWZ+` flag and continuously concatenate all subsequent matching tag values blindly in the correct order into one text value until interrupted by a standard trailing tag like `?30` (BIC) or `?31` (IBAN).

### creditor_name

The `creditor_name` field shall be extracted from `:86:?32...`.

### creditor_iban

The `creditor_iban` field shall be extracted from `:86:?31...`.

### creditor_bic

The `creditor_bic` field shall be extracted from `:86:?30...`.

### debtor fields

The source account in `:25:` identifies the account from which the transaction statement originates.

Because `:25:` usually contains a local account number representation instead of a full IBAN, the exact mapping must be defined by the implementation.

Possible handling:
- map `:25:` to the account configured by `bank_account_id`
- optionally derive `debtor_iban` from account configuration
- optionally derive `debtor_name` and `debtor_bic` from account metadata rather than from the MTA file itself

## Notes on MTA Subfields

The `:86:` field contains structured subfields separated by markers such as:

- `?00`
- `?20`
- `?21`
- `?22`
- `?23`
- `?24`
- `?25`
- `?26`
- `?27`
- `?28`
- `?29`
- `?30`
- `?31`
- `?32`

These subfields may continue across multiple physical lines.  
The importer shall join continuation lines before parsing subfields.

Typical meanings in the provided examples are:

| Subfield | Meaning |
|----------|---------|
| `?00` | transaction type |
| `?20`–`?29` | reference and remittance-related text |
| `?30` | counterpart BIC |
| `?31` | counterpart IBAN |
| `?32` | counterpart name |

## Notes on Unmapped MTA Content

The MTA data may contain additional structured values that are not currently represented in the target table, including:

- statement reference from `:20:`
- statement number from `:28C:`
- opening balance from `:60F:`
- closing balance from `:62F:`
- EREF
- KREF
- MREF
- CRED
- PURP

These fields shall be ignored unless the schema is extended.

## Data Conversion Rules

The importer shall apply the following transformations:

### Date
- Source format in `:61:`: `YYMMDD`
- Target format: `YYYY-MM-DD`
- Example:
  - source: `240110`
  - target: `2024-01-10`

### Amount
- Source format uses a comma as decimal separator
- `DR` values shall be stored as negative integers in minor currency units
- `CR` values shall be stored as positive integers in minor currency units
- Example:
  - source: `DR46,55`
  - database (`amount_minor`): `-4655`
  - source: `CR28,00`
  - database (`amount_minor`): `2800`

### Empty Values
- Missing optional values shall be stored as `NULL` for nullable database columns.

### Multi-line detail fields
- `:86:` continuation lines shall be merged before parsing
- split remittance text shall be concatenated into one string

## Provider Tracking

Each imported transaction must also insert a row into the `TRANSACTION_PROVIDERS` table linking the transaction to the MTA file's provider. This allows tracking of which provider sourced each transaction and enables reconciliation workflows where transactions from multiple sources may need to be cross-referenced.

## Functional Requirements

1. The system shall recursively scan the configured data folder for MTA files matching the expected directory structure.
2. The system shall parse MTA files as tagged text files.
3. The system shall split files into transaction blocks separating periods.
4. The system shall extract one transaction record per `:61:` / `:86:` pair dynamically, supporting multiple sequential pairs clustered within a single `-` statement block.
5. The system shall parse `DR` and `CR` indicators correctly to determine the sign of `amount_minor`.
6. The system shall convert amounts to integer minor currency units (e.g. cents).
7. The system shall extract `currency_code` from the statement balance information.
8. The system shall reconstruct structured `:86:` data across line breaks and concatenate continuous remittance subfields split across arbitrary `?2x` lengths.
9. The system shall transform and import each transaction into the `transactions` table.
10. The system shall store missing optional values as `NULL`.
11. The system shall log or report transactions that cannot be imported due to invalid format or missing required values.
12. The system should continue processing remaining transactions if a single transaction fails, unless configured otherwise.
13. The system shall insert a corresponding row into `TRANSACTION_PROVIDERS` for each imported transaction, linking it to the MTA file's provider.

## Parsing Rules

The importer shall use the following parsing rules for the provided MTA variant:

1. A line containing only `-` starts a new statement block.
2. `:61:` contains the transaction date and amount.
3. `:86:` contains structured details for the preceding `:61:` transaction.
4. Lines beginning with `?nn` after `:86:` belong to the same `:86:` content.
5. `?30`, `?31`, and `?32` shall be parsed as BIC, IBAN, and name of the counterpart if present.
6. `SVWZ+` content shall be intelligently tracked; once detected, all subsequent `?2x` and `?6x` tag values must be blindly concatenated into the overall remittance text until the end of the tag set or a known entity tag (e.g., `?30`) resets the context.
7. If `:86:` is missing, the transaction may still be imported with only the data available from `:61:`.

## Open Questions

The following points should be clarified before implementation:

1. **How is `bank_account_id` determined?**  
   The target table requires `bank_account_id`, but the MTA file does not directly provide it in a usable database form.

2. **How should `:25:` be mapped?**  
   The source account may not be given as IBAN, but as bank code/account number.

3. **Should the debtor/creditor roles depend on transaction direction?**  
   For outgoing transactions (`DR`), the counterpart is usually the creditor.  
   For incoming transactions (`CR`), the counterpart may semantically be the debtor.  
   If a strict accounting model is required, the mapping rules may need to depend on transaction type.

4. **Should duplicate imports be prevented?**  
   If the same MTA file is imported twice, should duplicate transactions be inserted or skipped?

5. **Should `balance_after_minor` be populated from MTA data?**  
   The MTA file contains opening (`:60F:`) and closing (`:62F:`) balances per statement block. The balance after each transaction could be derived if needed.

## Suggested Acceptance Criteria

- Given a valid MTA file in the expected folder structure, the importer inserts all valid transactions into `transactions`.
- The importer correctly extracts `booking_date`, `amount_minor`, `currency_code`, and transaction details from the MTA content.
- Debit transactions are stored with negative `amount_minor`.
- Credit transactions are stored with positive `amount_minor`.
- Dates are stored in `YYYY-MM-DD` format.
- Amounts are stored as integer minor currency units (e.g. `-4655` for `-46.55 EUR`).
- Structured `:86:` content is correctly reconstructed across multiple lines.
- Counterpart name, IBAN, and BIC are extracted when present.
- Remittance information is concatenated correctly from split `SVWZ+` fragments.
- Missing optional fields are stored as `NULL`.
- Invalid transactions are reported with enough detail to diagnose the issue.
