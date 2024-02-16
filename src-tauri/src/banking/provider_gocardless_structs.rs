use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access: String,
    access_expires: u64,
    refresh: String,
    refresh_expires: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    pub id: String,
    pub name: String,
    bic: String,
    transaction_total_days: String,
    countries: Vec<String>,
    logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    short: String,
    long: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankConnection {
    pub id: Option<String>,
    created: Option<String>,
    redirect: Option<String>,
    status: Option<String>,
    institution_id: String,
    agreement: Option<String>,
    reference: Option<String>,
    accounts: Option<Vec<HashMap<String, String>>>,
    user_language: Option<String>,
    pub link: Option<String>,
    ssn: Option<String>,
    account_selection: Option<bool>,
    redirect_immediate: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccounts {
    id: String,
    status: String,
    agreements: Option<String>,
    pub accounts: Vec<String>,
    reference: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionAmount {
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DebtorAccount {
    pub iban: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreditorAccount {
    pub iban: Option<String>,
    bban: Option<String>,
    pan: Option<String>,
    masked_pan: Option<String>,
    msisdn: Option<String>,
    currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: Option<String>,
    pub debtor_name: Option<String>,
    pub creditor_name: Option<String>,
    pub debtor_account: Option<DebtorAccount>,
    pub creditor_account: Option<CreditorAccount>,
    pub transaction_amount: TransactionAmount,
    pub booking_date: String,
    pub value_date: String,
    pub remittance_information_unstructured: Option<String>,
    pub bank_transaction_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub booked: Vec<Transaction>,
    pub pending: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankTransactions {
    pub transactions: Transactions,
}

#[derive(Debug)]
pub struct GoCardless {
    pub base_url: String,
    pub secret_id: String,
    pub secret_key: String,
    pub access_token: Option<AccessToken>,
}
