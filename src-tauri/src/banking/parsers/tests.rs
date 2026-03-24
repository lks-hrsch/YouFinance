use crate::model::NewTransaction;
use super::csv::parse_csv;
use super::mta::parse_mta;
use std::io::Cursor;

// We will construct identical representations of the same underlying transaction 
// in both CSV and MT940 format to test parser unification as requested by the user.

pub const UNIFIED_CSV: &str = "Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Gekennzeichneter Umsatz;Glaeubiger ID;Mandatsreferenz\n\
EINZUGSERMAECHTIGUNG;00000000;BANKDEFFXXX;Sample Bank;10.01.2024;10.01.2024;Example Payee GmbH;DE00XXXXXXXXXXXXXXX;BANKDEFFXXX;LASTSCHRIFT;Monthly subscription;-46,55;EUR;;;;;";

pub const UNIFIED_MTA: &str = "-
:20:STARTUMS
:25:00000000
:60F:C240110EUR1367,23
:61:2401100110DR46,55NDDTKREF+
:86:105?00EINZUGSERMAECHTIGUNG?20EREF+XXXXXXXXXXXX
?30BANKDEFFXXX?31DE00XXXXXXXXXXXXXXX?32Example Payee GmbH
?24SVWZ+Monthly subscription
:62F:C240110EUR1320,68
-";

fn assert_unified_transaction(tx: &NewTransaction) {
    assert_eq!(tx.title, "EINZUGSERMAECHTIGUNG");
    assert_eq!(tx.creditor_name.as_deref(), Some("Example Payee GmbH"));
    assert_eq!(tx.creditor_iban.as_deref(), Some("DE00XXXXXXXXXXXXXXX"));
    assert_eq!(tx.creditor_bic.as_deref(), Some("BANKDEFFXXX"));
    assert_eq!(tx.amount, -46.55);
    assert_eq!(tx.currency, "EUR");
    assert_eq!(tx.date, "10.01.2024");
    assert_eq!(tx.remittance_information.as_deref(), Some("Monthly subscription"));
    assert_eq!(tx.account_id, 99);
}

#[test]
fn test_unified_csv_parser() {
    let reader = Cursor::new(UNIFIED_CSV);
    let result = parse_csv(reader, 99).expect("CSV Parsing failed");
    assert_eq!(result.len(), 1);
    assert_unified_transaction(&result[0]);
    // CSV specifically parses debitor bank name which MTA typically ignores
    assert_eq!(result[0].debitor_name.as_deref(), Some("Sample Bank"));
}

#[test]
fn test_unified_mta_parser() {
    let reader = Cursor::new(UNIFIED_MTA);
    let result = parse_mta(reader, 99).expect("MTA Parsing failed");
    assert_eq!(result.len(), 1);
    assert_unified_transaction(&result[0]);
    // MTA typically maps :25: into debitor_iban 
    assert_eq!(result[0].debitor_iban.as_deref(), Some("00000000"));
}
