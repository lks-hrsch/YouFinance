use serde::Deserialize;
use std::io::Read;

use crate::model::NewTransaction;

#[derive(Debug, Deserialize)]
pub struct CsvTransactionRow {
    #[serde(rename = "Bezeichnung Auftragskonto")]
    pub title: String,
    #[serde(rename = "IBAN Auftragskonto")]
    pub debitor_iban: Option<String>,
    #[serde(rename = "BIC Auftragskonto")]
    pub debitor_bic: Option<String>,
    #[serde(rename = "Bankname Auftragskonto")]
    pub debitor_name: Option<String>,
    #[serde(rename = "Buchungstag")]
    pub date: String,
    #[serde(rename = "Name Zahlungsbeteiligter")]
    pub creditor_name: Option<String>,
    #[serde(rename = "IBAN Zahlungsbeteiligter")]
    pub creditor_iban: Option<String>,
    #[serde(rename = "BIC (SWIFT-Code) Zahlungsbeteiligter")]
    pub creditor_bic: Option<String>,
    #[serde(rename = "Verwendungszweck")]
    pub remittance_information: Option<String>,
    #[serde(rename = "Betrag")]
    pub amount: String,
    #[serde(rename = "Waehrung")]
    pub currency: String,
}

pub fn parse_csv<R: Read>(reader: R, account_id: i32) -> Result<Vec<NewTransaction>, String> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(reader);

    let mut transactions = Vec::new();

    for result in rdr.deserialize() {
        let record: CsvTransactionRow = result.map_err(|e| format!("CSV Parse Error: {}", e))?;
        
        let mut amount_str = record.amount.clone();
        amount_str = amount_str.replace('.', ""); // Replace thousand separators
        amount_str = amount_str.replace(',', "."); // Replace decimal comma with dot
        
        let amount_f64: f64 = amount_str.parse()
            .map_err(|e| format!("Invalid amount format '{}': {}", record.amount, e))?;

        let new_tx = NewTransaction {
            title: record.title,
            debitor_name: none_if_empty(record.debitor_name),
            debitor_iban: none_if_empty(record.debitor_iban),
            debitor_bic: none_if_empty(record.debitor_bic),
            creditor_name: none_if_empty(record.creditor_name),
            creditor_iban: none_if_empty(record.creditor_iban),
            creditor_bic: none_if_empty(record.creditor_bic),
            amount: amount_f64,
            currency: record.currency,
            date: record.date,
            remittance_information: none_if_empty(record.remittance_information),
            account_id,
        };

        transactions.push(new_tx);
    }
    
    Ok(transactions)
}

fn none_if_empty(s: Option<String>) -> Option<String> {
    s.filter(|v| !v.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use rstest::rstest;

    #[rstest]
    #[case::standard_valid(
        "Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Gekennzeichneter Umsatz;Glaeubiger ID;Mandatsreferenz\n\
        BusinessAccount;DE00XXXX00000000000000;BANKDEFFXXX;Sample Bank;30.12.2024;30.12.2024;Telecom Provider GmbH;DE00XXXX00000000000001;HYVEDEMMXXX;LASTSCHRIFT;Customer No.: XXXXXXXX, Invoice No.: XXXXXXXXXX, Monthly service charge EREF: XXXXXXXXXXXXXXXXXXXXXXXXXXXX MREF: XXXXXXXXXXXXXXXXXXXXXXXXXXXX CRED: DE97XXXXXXXXXXXXXXX IBAN: DE00XXXX00000000000001 BIC: HYVEDEMMXXX;-17,49;EUR;1853,02;;;DE97XXXXXXXXXXXXXXX;XXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        42,
        "BusinessAccount",
        -17.49,
        "30.12.2024",
        Some("Telecom Provider GmbH"),
        Some("HYVEDEMMXXX"),
        Some("DE00XXXX00000000000001"),
        Some("DE00XXXX00000000000000"),
        Some("BANKDEFFXXX"),
        Some("Sample Bank")
    )]
    #[case::empty_fields_and_thousand_separators(
        "Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Gekennzeichneter Umsatz;Glaeubiger ID;Mandatsreferenz\n\
        BusinessAccount;;;;30.12.2024;30.12.2024;;;;;some info;1.000,50;USD;;;;;",
        1,
        "BusinessAccount",
        1000.50,
        "30.12.2024",
        None,
        None,
        None,
        None,
        None,
        None
    )]
    fn test_parse_csv_parameterized(
        #[case] csv_data: &str,
        #[case] expected_account: i32,
        #[case] expected_title: &str,
        #[case] expected_amount: f64,
        #[case] expected_date: &str,
        #[case] expected_creditor_name: Option<&str>,
        #[case] expected_creditor_bic: Option<&str>,
        #[case] expected_creditor_iban: Option<&str>,
        #[case] expected_debitor_iban: Option<&str>,
        #[case] expected_debitor_bic: Option<&str>,
        #[case] expected_debitor_name: Option<&str>,
    ) {
        let reader = Cursor::new(csv_data);
        let result = parse_csv(reader, expected_account).unwrap();
        assert_eq!(result.len(), 1);
        let tx = &result[0];

        assert_eq!(tx.title, expected_title);
        assert_eq!(tx.amount, expected_amount);
        assert_eq!(tx.date, expected_date);
        assert_eq!(tx.creditor_name.as_deref(), expected_creditor_name);
        assert_eq!(tx.creditor_bic.as_deref(), expected_creditor_bic);
        assert_eq!(tx.creditor_iban.as_deref(), expected_creditor_iban);
        assert_eq!(tx.debitor_iban.as_deref(), expected_debitor_iban);
        assert_eq!(tx.debitor_bic.as_deref(), expected_debitor_bic);
        assert_eq!(tx.debitor_name.as_deref(), expected_debitor_name);
        assert_eq!(tx.account_id, expected_account);
    }
}
