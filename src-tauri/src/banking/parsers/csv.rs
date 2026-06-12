use serde::Deserialize;
use std::io::Read;

use crate::model::NewTransaction;
use crate::banking::utils::{normalize_date, none_if_empty};

#[derive(Debug, Deserialize)]
pub struct CsvTransactionRow {
    #[serde(rename = "Buchungstext")]
    pub booking_text: String,
    #[serde(rename = "IBAN Auftragskonto")]
    pub debtor_iban: Option<String>,
    #[serde(rename = "BIC Auftragskonto")]
    pub debtor_bic: Option<String>,
    #[serde(rename = "Bezeichnung Auftragskonto")]
    pub debtor_name: Option<String>,
    #[serde(rename = "Buchungstag")]
    pub booking_date: String,
    #[serde(rename = "Valutadatum")]
    pub value_date: Option<String>,
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
    #[serde(rename = "Saldo nach Buchung")]
    pub balance_after_string: Option<String>,
    #[serde(rename = "Mandatsreferenz")]
    pub mandate_reference: Option<String>,
}

pub fn parse_csv<R: Read>(reader: R, bank_account_id: i32) -> Result<Vec<NewTransaction>, String> {
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

        let amount_minor: i32 = (amount_f64 * 100.0).round() as i32;

        // Parse balance_after if present
        let balance_after_minor = record.balance_after_string.as_ref().and_then(|s| {
            let cleaned = s.replace('.', "").replace(',', ".");
            cleaned.parse::<f64>().ok().map(|f| (f * 100.0).round() as i32)
        });

        let new_tx = NewTransaction {
            booking_text: record.booking_text,
            debtor_name: none_if_empty(record.debtor_name),
            debtor_iban: none_if_empty(record.debtor_iban),
            debtor_bic: none_if_empty(record.debtor_bic),
            creditor_name: none_if_empty(record.creditor_name),
            creditor_iban: none_if_empty(record.creditor_iban),
            creditor_bic: none_if_empty(record.creditor_bic),
            amount_minor,
            currency_code: record.currency,
            booking_date: normalize_date(&record.booking_date),
            value_date: record.value_date.as_ref().map(|d| normalize_date(d)),
            balance_after_minor,
            mandate_reference: none_if_empty(record.mandate_reference),
            remittance_information: none_if_empty(record.remittance_information),
            bank_account_id,
        };

        transactions.push(new_tx);
    }

    Ok(transactions)
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
        "LASTSCHRIFT",
        -1749,
        "2024-12-30",
        Some("Telecom Provider GmbH"),
        Some("HYVEDEMMXXX"),
        Some("DE00XXXX00000000000001"),
        Some("DE00XXXX00000000000000"),
        Some("BANKDEFFXXX"),
        Some("BusinessAccount"),
        Some("XXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
    )]
    #[case::empty_fields_and_thousand_separators(
        "Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Gekennzeichneter Umsatz;Glaeubiger ID;Mandatsreferenz\n\
        BusinessAccount;;;;30.12.2024;30.12.2024;;;;;some info;1.000,50;USD;;;;;",
        1,
        "",
        100050,
        "2024-12-30",
        None,
        None,
        None,
        None,
        None,
        Some("BusinessAccount"),
        None,
    )]
    fn test_parse_csv_parameterized(
        #[case] csv_data: &str,
        #[case] expected_account: i32,
        #[case] expected_booking_text: &str,
        #[case] expected_amount_minor: i32,
        #[case] expected_booking_date: &str,
        #[case] expected_creditor_name: Option<&str>,
        #[case] expected_creditor_bic: Option<&str>,
        #[case] expected_creditor_iban: Option<&str>,
        #[case] expected_debtor_iban: Option<&str>,
        #[case] expected_debtor_bic: Option<&str>,
        #[case] expected_debtor_name: Option<&str>,
        #[case] expected_mandate_reference: Option<&str>,
    ) {
        let reader = Cursor::new(csv_data);
        let result = parse_csv(reader, expected_account).unwrap();
        assert_eq!(result.len(), 1);
        let tx = &result[0];

        assert_eq!(tx.booking_text, expected_booking_text);
        assert_eq!(tx.amount_minor, expected_amount_minor);
        assert_eq!(tx.booking_date, expected_booking_date);
        assert_eq!(tx.creditor_name.as_deref(), expected_creditor_name);
        assert_eq!(tx.creditor_bic.as_deref(), expected_creditor_bic);
        assert_eq!(tx.creditor_iban.as_deref(), expected_creditor_iban);
        assert_eq!(tx.debtor_iban.as_deref(), expected_debtor_iban);
        assert_eq!(tx.debtor_bic.as_deref(), expected_debtor_bic);
        assert_eq!(tx.debtor_name.as_deref(), expected_debtor_name);
        assert_eq!(tx.mandate_reference.as_deref(), expected_mandate_reference);
        assert_eq!(tx.bank_account_id, expected_account);
    }
}
