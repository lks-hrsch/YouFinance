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

        let mut account_number = None;
        let mut currency = "EUR".to_string();
        
        let mut raw_tx_tags = Vec::new();
        let mut current_tag = String::new();
        let mut current_val = String::new();
        
        let mut process_tag = |tag: &str, val: &str, raw_list: &mut Vec<(String, String)>| {
            if tag == "25" {
                account_number = Some(val.to_string());
            } else if tag == "60F" || tag == "62F" {
                if val.len() >= 10 {
                    currency = val[7..10].to_string();
                }
            } else if tag == "61" || tag == "86" {
                raw_list.push((tag.to_string(), val.to_string()));
            }
        };

        for line in block.lines() {
            if line.starts_with(':') {
                if let Some(end_idx) = line[1..].find(':') {
                    if !current_tag.is_empty() {
                        process_tag(&current_tag, &current_val, &mut raw_tx_tags);
                    }
                    current_tag = line[1..=end_idx].to_string();
                    current_val = line[end_idx + 2..].to_string();
                } else {
                    current_val.push_str(line);
                }
            } else {
                if !current_tag.is_empty() {
                    current_val.push_str(line);
                }
            }
        }
        if !current_tag.is_empty() {
            process_tag(&current_tag, &current_val, &mut raw_tx_tags);
        }

        let mut pending_61 = None;
        for (tag, val) in raw_tx_tags {
            if tag == "61" {
                pending_61 = Some(val);
            } else if tag == "86" {
                if let Some(tag61) = pending_61.take() {
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
                    } else if let Some(rc_idx) = tag61.find("RC") {
                        is_dr = false;
                        amount_start = rc_idx + 2;
                    } else if let Some(rd_idx) = tag61.find("RD") {
                        is_dr = true;
                        amount_start = rd_idx + 2;
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
                    let mut creditor_name: Option<String> = None;
                    let mut creditor_iban: Option<String> = None;
                    let mut creditor_bic: Option<String> = None;
                    let mut remittance = String::new();
                    let mut in_remittance = false;

                    let parts: Vec<&str> = val.split('?').collect();
                    for part in parts {
                        if part.len() < 2 { continue; }
                        let code = &part[0..2];
                        let part_val = &part[2..];

                        if code == "00" {
                            title = part_val.to_string();
                        } else if code == "30" {
                            creditor_bic = Some(part_val.to_string());
                            in_remittance = false;
                        } else if code == "31" {
                            creditor_iban = Some(part_val.to_string());
                            in_remittance = false;
                        } else if code == "32" || code == "33" {
                            if let Some(existing) = creditor_name.as_mut() {
                                existing.push_str(part_val);
                            } else {
                                creditor_name = Some(part_val.to_string());
                            }
                            in_remittance = false;
                        } else if code.starts_with('2') || code.starts_with('6') {
                            if let Some(idx) = part_val.find("SVWZ+") {
                                in_remittance = true;
                                if !remittance.is_empty() {
                                    remittance.push(' ');
                                }
                                remittance.push_str(&part_val[idx + 5..]);
                            } else if in_remittance {
                                remittance.push_str(part_val);
                            }
                        } else {
                            in_remittance = false;
                        }
                    }

                    let new_tx = NewTransaction {
                        title,
                        debitor_name: None,
                        debitor_iban: account_number.clone(),
                        debitor_bic: None,
                        creditor_name,
                        creditor_iban,
                        creditor_bic,
                        amount: amount_val,
                        currency: currency.clone(),
                        date: date_str,
                        remittance_information: if remittance.is_empty() { None } else { Some(remittance) },
                        account_id,
                    };

                    transactions.push(new_tx);
                }
            }
        }
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
        "-
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
-",
        1,
        "EINZUGSERMAECHTIGUNG",
        -46.55,
        "10.01.2024",
        Some("Example Payee GmbH"),
        Some("BANKDEFFXXX"),
        Some("DE00XXXXXXXXXXXXXXX")
    )]
    #[case::credit_missing_tags(
        "-
:61:2412311231CR1234,56NTRFNONREF
:86:?00LOHN/GEHALT?32My Employer
-",
        2,
        "LOHN/GEHALT",
        1234.56,
        "31.12.2024",
        Some("My Employer"),
        None,
        None
    )]
    #[case::multi_line_remittance_anonymized(
        "-
:20:STARTUMS
:61:2401100110DR46,55NDDTKREF+
:86:105?00EINZUGSERMAECHTIGUNG?20EREF+Beleg-Nr.123
?23SVWZ+Konsumausgaben + Datea
?24bend + Geschenke (EXT-KONSU?25M) /.DA-2. IBAN: DE00000000
?26000000000000 BIC: EXAMXXX?27XX?30EXAMXXX
?31DE00000000000000000000?32Max Mustermann
-",
        1,
        "EINZUGSERMAECHTIGUNG",
        -46.55,
        "10.01.2024",
        Some("Max Mustermann"),
        Some("EXAMXXX"),
        Some("DE00000000000000000000")
    )]
    fn test_parse_mta_parameterized(
        #[case] mta_data: &str,
        #[case] expected_account: i32,
        #[case] expected_title: &str,
        #[case] expected_amount: f64,
        #[case] expected_date: &str,
        #[case] expected_name: Option<&str>,
        #[case] expected_bic: Option<&str>,
        #[case] expected_iban: Option<&str>,
    ) {
        let reader = Cursor::new(mta_data);
        let result = parse_mta(reader, expected_account).unwrap();
        
        let tx = &result[0]; // Assuming each parameterized case focuses on the first or only tx
        assert_eq!(tx.title, expected_title);
        assert_eq!(tx.amount, expected_amount);
        assert_eq!(tx.date, expected_date);
        assert_eq!(tx.creditor_name.as_deref(), expected_name);
        assert_eq!(tx.creditor_bic.as_deref(), expected_bic);
        assert_eq!(tx.creditor_iban.as_deref(), expected_iban);
        assert_eq!(tx.account_id, expected_account);
    }

    #[test]
    fn test_parse_mta_multiple_transactions_per_block() {
        let mta_data = "-
:20:STARTUMS
:25:12345678/1111111111
:28C:0
:60F:C240110EUR1367,23
:61:2401100110DR46,55NDDTKREF+
:86:105?00EINZUGSERMAECHTIGUNG?30BANKDEFFXXX?31DE00XXXXXXXXXXXXXXX?32Example Payee GmbH
:61:2401100110CR10,00NTRFKREF+
:86:166?00GUTSCHRIFT?32A Friend
:62F:C240110EUR1330,68
-";
        let reader = Cursor::new(mta_data);
        let result = parse_mta(reader, 1).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].amount, -46.55);
        assert_eq!(result[0].creditor_name.as_deref(), Some("Example Payee GmbH"));
        assert_eq!(result[1].amount, 10.0);
        assert_eq!(result[1].creditor_name.as_deref(), Some("A Friend"));
    }
}
