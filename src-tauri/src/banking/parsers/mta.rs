use crate::model::NewTransaction;
use std::io::Read;

pub fn parse_mta<R: Read>(mut reader: R, account_id: i32) -> Result<Vec<NewTransaction>, String> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    // Unify line endings to \n
    let contents = contents.replace("\r\n", "\n");
    // Split into blocks by lines containing only "-" or "- " etc. We split by "\n-\n"
    // Also prepend and append \n to handle if file starts/ends with - immediately
    let search_content = format!("\n{}\n", contents);
    let blocks: Vec<&str> = search_content.split("\n-\n").collect();

    let mut transactions = Vec::new();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() || block == "-" {
            continue;
        }

        let mut tags: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut current_tag = String::new();

        for line in block.lines() {
            if line.starts_with(':') {
                if let Some(end_idx) = line[1..].find(':') {
                    let tag_name = &line[1..=end_idx];
                    let tag_value = &line[end_idx + 2..];
                    current_tag = tag_name.to_string();
                    tags.insert(current_tag.clone(), tag_value.to_string());
                } else {
                    if let Some(val) = tags.get_mut(&current_tag) {
                        val.push_str(line);
                    }
                }
            } else {
                if !current_tag.is_empty() {
                    if let Some(val) = tags.get_mut(&current_tag) {
                        val.push_str(line);
                    }
                }
            }
        }

        // Transactions must have a :61:
        let tag61 = match tags.get("61") {
            Some(v) => v,
            None => continue,
        };

        // Output formatting
        let mut currency = "EUR".to_string();
        if let Some(tag60) = tags.get("60F") {
            if tag60.len() >= 10 {
                currency = tag60[7..10].to_string();
            }
        } else if let Some(tag62) = tags.get("62F") {
            if tag62.len() >= 10 {
                currency = tag62[7..10].to_string();
            }
        }

        if tag61.len() < 10 { continue; }
        
        let yy = &tag61[0..2];
        let mm = &tag61[2..4];
        let dd = &tag61[4..6];
        let date_str = format!("{}.{}.20{}", dd, mm, yy);

        let mut is_dr = true;
        let mut amount_start = 6;
        if let Some(dr_idx) = tag61.find("DR") {
            is_dr = true;
            amount_start = dr_idx + 2;
        } else if let Some(cr_idx) = tag61.find("CR") {
            is_dr = false;
            amount_start = cr_idx + 2;
        }

        let mut amount_end = amount_start;
        for c in tag61[amount_start..].chars() {
            if c.is_ascii_digit() || c == ',' {
                amount_end += 1;
            } else {
                break;
            }
        }

        let mut amount_val: f64 = 0.0;
        if amount_end > amount_start {
            let amt_str = tag61[amount_start..amount_end].replace(',', ".");
            if let Ok(v) = amt_str.parse::<f64>() {
                amount_val = if is_dr { -v } else { v };
            }
        }

        let mut title = String::new();
        let mut creditor_name = None;
        let mut creditor_iban = None;
        let mut creditor_bic = None;
        let mut remittance = String::new();

        if let Some(tag86) = tags.get("86") {
            let parts: Vec<&str> = tag86.split('?').collect();
            for part in parts {
                if part.len() < 2 { continue; }
                let code = &part[0..2];
                let val = &part[2..];

                if code == "00" {
                    title = val.to_string();
                } else if code == "30" {
                    creditor_bic = Some(val.to_string());
                } else if code == "31" {
                    creditor_iban = Some(val.to_string());
                } else if code == "32" {
                    creditor_name = Some(val.to_string());
                } else if code.starts_with('2') {
                    if let Some(idx) = val.find("SVWZ+") {
                        let svwz = &val[idx + 5..];
                        if !remittance.is_empty() {
                            remittance.push_str(" ");
                        }
                        remittance.push_str(svwz);
                    }
                }
            }
        }

        let new_tx = NewTransaction {
            title,
            debitor_name: None,
            debitor_iban: tags.get("25").cloned(),
            debitor_bic: None,
            creditor_name,
            creditor_iban,
            creditor_bic,
            amount: amount_val,
            currency,
            date: date_str,
            remittance_information: if remittance.is_empty() { None } else { Some(remittance) },
            account_id,
        };

        transactions.push(new_tx);
    }

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_mta_valid() {
        let mta_data = "-
:20:STARTUMS
:25:12345678/1111111111
:28C:0
:60F:C240110EUR1367,23
:61:2401100110DR46,55NDDTKREF+
:86:105?00EINZUGSERMAECHTIGUNG?20EREF+XXXXXXXXXXXX
?21KREF+XXXXXXXXXXXXXXXXXXXX?22MREF+XXXXXXXXXXXX
?30BANKDEFFXXX?31DE00XXXXXXXXXXXXXXX?32Example Payee GmbH
?24SVWZ+Monthly subscription
:62F:C240110EUR1320,68
-";
        let reader = Cursor::new(mta_data);
        let result = parse_mta(reader, 1).unwrap();
        assert_eq!(result.len(), 1);
        let tx = &result[0];
        
        assert_eq!(tx.title, "EINZUGSERMAECHTIGUNG");
        assert_eq!(tx.debitor_iban.as_deref(), Some("12345678/1111111111"));
        assert_eq!(tx.date, "10.01.2024");
        assert_eq!(tx.creditor_name.as_deref(), Some("Example Payee GmbH"));
        assert_eq!(tx.creditor_iban.as_deref(), Some("DE00XXXXXXXXXXXXXXX"));
        assert_eq!(tx.creditor_bic.as_deref(), Some("BANKDEFFXXX"));
        assert_eq!(tx.amount, -46.55);
        assert_eq!(tx.currency, "EUR");
        assert_eq!(tx.account_id, 1);
        assert_eq!(tx.remittance_information.as_deref(), Some("Monthly subscription"));
    }

    #[test]
    fn test_parse_mta_credit_and_missing_tags() {
        let mta_data = "-
:61:2412311231CR1234,56NTRFNONREF
:86:?00LOHN/GEHALT?32My Employer
-";
        let reader = Cursor::new(mta_data);
        let result = parse_mta(reader, 2).unwrap();
        assert_eq!(result.len(), 1);
        let tx = &result[0];
        
        assert_eq!(tx.title, "LOHN/GEHALT");
        assert_eq!(tx.debitor_iban, None); // no :25: provided
        assert_eq!(tx.date, "31.12.2024");
        assert_eq!(tx.amount, 1234.56); // positive CR
        assert_eq!(tx.currency, "EUR"); // default when missing
        assert_eq!(tx.creditor_name.as_deref(), Some("My Employer"));
        assert_eq!(tx.remittance_information, None);
    }
}
