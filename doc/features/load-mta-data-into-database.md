# load mta data into the database

The system shall support importing transaction data from MTA files into the database.  
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
| `:25:` account identification | debitor_iban or source account mapping |
| `:61:` booking date | date |
| `:61:` amount | amount |
| `:61:` currency inferred from surrounding statement block (`:60F:` / `:62F:`) | currency |
| `:86:?00...` transaction type text | title |
| `:86:?32...` counterpart name | creditor_name |
| `:86:?31...` counterpart IBAN | creditor_iban |
| `:86:?30...` counterpart BIC | creditor_bic |
| `:86:?2x...SVWZ+...` remittance information | remittance_information |

## Interpretation Rules

Because MTA files do not provide the fields in exactly the same shape as the CSV import, the importer shall interpret the data as follows.

### title

The `title` field shall be filled from the transaction type in the `:86:` segment.

Examples:
- `EINZUGSERMAECHTIGUNG`
- `GUTSCHRIFT`
- `SEPA-Ueberweisung`

This is usually encoded in `?00`.

### date

The `date` field shall be taken from the `:61:` tag.

Example:

```text
:61:2401100110DR46,55NDDTKREF+
```

The booking date is `240110`, which corresponds to:

```text
10.01.2024
```

The importer shall convert the date to the format:

```text
DD.MM.YYYY
```

### amount

The `amount` field shall be taken from the `:61:` tag.

Examples:
- `DR46,55` means `-46.55`
- `CR28,00` means `28.00`

Rules:
- `DR` indicates a debit and shall be stored as a negative number
- `CR` indicates a credit and shall be stored as a positive number
- the comma decimal separator shall be converted to a dot for storage as `Double`

### currency

The `currency` field shall be determined from the statement currency in `:60F:` or `:62F:`.

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

### debitor fields

The source account in `:25:` identifies the account from which the transaction statement originates.

Because `:25:` usually contains a local account number representation instead of a full IBAN, the exact mapping must be defined by the implementation.

Possible handling:
- map `:25:` to the account configured by `account_id`
- optionally derive `debitor_iban` from account configuration
- optionally derive `debitor_name` and `debitor_bic` from account metadata rather than from the MTA file itself

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
- Target format: `DD.MM.YYYY`
- Example:
  - source: `240110`
  - target: `10.01.2024`

### Amount
- Source format uses a comma as decimal separator
- `DR` values shall be negative
- `CR` values shall be positive
- Example:
  - source: `DR46,55`
  - database: `-46.55`
  - source: `CR28,00`
  - database: `28.00`

### Empty Values
- Missing optional values shall be stored as `NULL` for nullable database columns.

### Multi-line detail fields
- `:86:` continuation lines shall be merged before parsing
- split remittance text shall be concatenated into one string

## Functional Requirements

1. The system shall recursively scan the configured data folder for MTA files matching the expected directory structure.
2. The system shall parse MTA files as tagged text files.
3. The system shall split files into transaction blocks separating periods.
4. The system shall extract one transaction record per `:61:` / `:86:` pair dynamically, supporting multiple sequential pairs clustered within a single `-` statement block.
5. The system shall parse `DR` and `CR` indicators correctly to determine the sign of the amount.
6. The system shall extract currency from the statement balance information.
7. The system shall reconstruct structured `:86:` data across line breaks and concatenate continuous remittance subfields split across arbitrary `?2x` lengths.
8. The system shall transform and import each transaction into the `transactions` table.
9. The system shall store missing optional values as `NULL`.
10. The system shall log or report transactions that cannot be imported due to invalid format or missing required values.
11. The system should continue processing remaining transactions if a single transaction fails, unless configured otherwise.

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

1. **How is `account_id` determined?**  
   The target table requires `account_id`, but the MTA file does not directly provide it in a usable database form.

2. **How should `:25:` be mapped?**  
   The source account may not be given as IBAN, but as bank code/account number.

3. **Should the debitor/creditor roles depend on transaction direction?**  
   For outgoing transactions (`DR`), the counterpart is usually the creditor.  
   For incoming transactions (`CR`), the counterpart may semantically be the debitor.  
   If a strict accounting model is required, the mapping rules may need to depend on transaction type.

4. **Should duplicate imports be prevented?**  
   If the same MTA file is imported twice, should duplicate transactions be inserted or skipped?

5. **Should balance information be stored in a separate table?**  
   The MTA file contains opening and closing balances that may be useful later.

## Suggested Acceptance Criteria

- Given a valid MTA file in the expected folder structure, the importer inserts all valid transactions into `transactions`.
- The importer correctly extracts date, amount, currency, and transaction details from the MTA content.
- Debit transactions are stored with negative amounts.
- Credit transactions are stored with positive amounts.
- Structured `:86:` content is correctly reconstructed across multiple lines.
- Counterpart name, IBAN, and BIC are extracted when present.
- Remittance information is concatenated correctly from split `SVWZ+` fragments.
- Missing optional fields are stored as `NULL`.
- Invalid transactions are reported with enough detail to diagnose the issue.
