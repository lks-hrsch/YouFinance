use crate::model::Transaction;

/// Represents the deduplication outcome when checking if a transaction
/// already exists in the database.
#[derive(Debug, PartialEq, Eq)]
pub enum DeduplicationOutcome {
    /// Transaction is new and should be inserted
    New,
    /// Transaction exists in the same account; check for conflicts before merging
    SameAccount { existing_id: i32 },
    /// Transaction exists in a different account; multi-source dedup
    CrossAccount {
        existing_id: i32,
        existing_account_id: i32,
    },
}

/// Pure function to classify the deduplication outcome based on:
/// - Whether a matching transaction exists
/// - Whether the match is in the same account or a different one
pub fn classify_dedup_outcome(
    maybe_existing: Option<&Transaction>,
    incoming_account_id: i32,
) -> DeduplicationOutcome {
    match maybe_existing {
        None => DeduplicationOutcome::New,
        Some(existing) => {
            let existing_id = existing.id;
            if existing.bank_account_id == incoming_account_id {
                DeduplicationOutcome::SameAccount { existing_id }
            } else {
                DeduplicationOutcome::CrossAccount {
                    existing_id,
                    existing_account_id: existing.bank_account_id,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction(id: i32, bank_account_id: i32) -> Transaction {
        Transaction {
            id,
            booking_text: "test".to_string(),
            debtor_name: None,
            debtor_iban: None,
            debtor_bic: None,
            creditor_name: None,
            creditor_iban: None,
            creditor_bic: None,
            amount_minor: 1000,
            currency_code: "EUR".to_string(),
            booking_date: "2026-03-27".to_string(),
            value_date: None,
            balance_after_minor: None,
            mandate_reference: None,
            remittance_information: None,
            bank_account_id,
            created_at: "2026-03-27T00:00:00Z".to_string(),
            updated_at: "2026-03-27T00:00:00Z".to_string(),
            deleted_at: None,
        }
    }

    #[test]
    fn no_match_yields_new() {
        let outcome = classify_dedup_outcome(None, 42);
        assert_eq!(outcome, DeduplicationOutcome::New);
    }

    #[test]
    fn same_account_match() {
        let existing = create_test_transaction(10, 42);
        let outcome = classify_dedup_outcome(Some(&existing), 42);
        assert_eq!(
            outcome,
            DeduplicationOutcome::SameAccount { existing_id: 10 }
        );
    }

    #[test]
    fn cross_account_match() {
        let existing = create_test_transaction(10, 1);
        let outcome = classify_dedup_outcome(Some(&existing), 2);
        assert_eq!(
            outcome,
            DeduplicationOutcome::CrossAccount {
                existing_id: 10,
                existing_account_id: 1
            }
        );
    }

    #[test]
    fn correct_existing_id_preserved() {
        let existing = create_test_transaction(99, 1);
        let outcome = classify_dedup_outcome(Some(&existing), 2);
        match outcome {
            DeduplicationOutcome::CrossAccount { existing_id, .. } => {
                assert_eq!(existing_id, 99);
            }
            _ => panic!("expected CrossAccount outcome"),
        }
    }
}
