use std::path::PathBuf;
use std::fs;
use tauri_plugin_log::log::{debug, error};

use super::super::{
    apierror::*,
    trait_banking_api::BankingApi,
};
use crate::banking::providers::gocardless::structs::*;
use crate::banking::parsers::mta::parse_mta;

pub struct LocalMTA {
    pub data_dir: PathBuf,
}

impl LocalMTA {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    /// Internal recursive search for MTA files associated with an account ID.
    fn find_mta_files_recursive(&self, dir: &std::path::Path, account_id: &str, depth: usize, mta_files: &mut Vec<PathBuf>) {
        if depth > 3 { return; }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().is_some_and(|n| n == account_id) {
                        // Found account dir, collect all MTAs inside it (unlimited depth here)
                        self.collect_all_mtas(&path, mta_files);
                    } else {
                        // Keep searching
                        self.find_mta_files_recursive(&path, account_id, depth + 1, mta_files);
                    }
                }
            }
        }
    }

    fn collect_all_mtas(&self, dir: &std::path::Path, mta_files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    self.collect_all_mtas(&path, mta_files);
                } else if path.is_file() && path.extension().is_some_and(|ext| ext == "mta") {
                    mta_files.push(path);
                }
            }
        }
    }
}

impl BankingApi for LocalMTA {
    async fn get_access_token(&self) -> Result<AccessToken, ApiError> {
        Err(ApiError::Custom("LocalMTA does not support access tokens".into()))
    }

    async fn get_banks_by_country(&self, _country: &str) -> Result<Vec<Bank>, ApiError> {
        Ok(vec![])
    }

    async fn connect_bank(&self, _redirect: &str, _institution_id: &str) -> Result<BankConnection, ApiError> {
        Err(ApiError::Custom("LocalMTA does not support connect_bank".into()))
    }

    async fn disconnect_bank(&self, _bank_connection_id: &str) -> Result<(), ApiError> {
        Ok(())
    }

    async fn get_bank_accounts(&self, _bank_connection_id: &str) -> Result<BankAccounts, ApiError> {
        Ok(BankAccounts {
            id: "local_mta".into(),
            status: "ready".into(),
            agreements: None,
            accounts: vec![],
            reference: "local".into(),
        })
    }

    async fn get_account_transactions(&self, account_id: &str) -> Result<BankTransactions, ApiError> {
        debug!("LocalMTA::get_account_transactions for account: {}", account_id);

        let mut booked_transactions = vec![];

        if self.data_dir.exists() {
            // Find all MTA files in the data directory recursively (max depth 3)
            let mut mta_files = Vec::new();
            self.find_mta_files_recursive(&self.data_dir, account_id, 0, &mut mta_files);

            for mta_path in mta_files {
                debug!("Parsing MTA file: {:?}", mta_path);
                let file = fs::File::open(&mta_path)
                    .map_err(|e| ApiError::Custom(format!("Failed to open MTA file: {}", e)))?;

                let new_transactions = parse_mta(file, 0)
                    .map_err(|e| ApiError::Custom(format!("Failed to parse MTA: {}", e)))?;

                for nt in new_transactions {
                    booked_transactions.push(Transaction {
                        transaction_id: None,
                        debtor_name: nt.debtor_name,
                        creditor_name: nt.creditor_name,
                        debtor_account: nt.debtor_iban.map(|iban| DebtorAccount { iban: Some(iban) }),
                        creditor_account: nt.creditor_iban.map(|iban| CreditorAccount {
                            iban: Some(iban),
                            bban: None,
                            pan: None,
                            masked_pan: None,
                            msisdn: None,
                            currency: None,
                        }),
                        transaction_amount: TransactionAmount {
                            currency: nt.currency_code,
                            amount: (nt.amount_minor as f64 / 100.0).to_string(),
                        },
                        booking_date: Some(nt.booking_date.clone()),
                        booking_date_time: None,
                        value_date: nt.value_date.clone(),
                        value_date_time: None,
                        remittance_information_unstructured: nt.remittance_information,
                        bank_transaction_code: Some(nt.booking_text),
                        balance_after_minor: nt.balance_after_minor,
                    });
                }
            }
        } else {
            error!("Data directory does not exist: {:?}", self.data_dir);
        }

        Ok(BankTransactions {
            transactions: Transactions {
                booked: booked_transactions,
                pending: vec![],
            },
        })
    }
}
