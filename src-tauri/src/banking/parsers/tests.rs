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

use rstest::rstest;

#[rstest]
#[case::unified_csv_parser(UNIFIED_CSV, "csv")]
#[case::unified_mta_parser(UNIFIED_MTA, "mta")]
fn test_unified_parsers(#[case] data: &str, #[case] parser_type: &str) {
    let reader = Cursor::new(data);
    
    let result = if parser_type == "csv" {
        parse_csv(reader, 99).expect("CSV Parsing failed")
    } else {
        parse_mta(reader, 99).expect("MTA Parsing failed")
    };
    
    assert_eq!(result.len(), 1);
    let tx = &result[0];

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
