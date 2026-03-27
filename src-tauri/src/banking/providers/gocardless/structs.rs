use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access: String,
    pub access_expires: u64,
    pub refresh: String,
    pub refresh_expires: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    pub id: String,
    pub name: String,
    pub bic: String,
    pub transaction_total_days: String,
    pub countries: Vec<String>,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub short: String,
    pub long: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankConnection {
    pub id: Option<String>,
    pub created: Option<String>,
    pub redirect: Option<String>,
    pub status: Option<String>,
    pub institution_id: String,
    pub agreement: Option<String>,
    pub reference: Option<String>,
    pub accounts: Option<Vec<String>>,
    pub user_language: Option<String>,
    pub link: Option<String>,
    pub ssn: Option<String>,
    pub account_selection: Option<bool>,
    pub redirect_immediate: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedBankConnection {
    pub count: Option<u64>,
    next: Option<String>,
    previous: Option<String>,
    pub results: Vec<BankConnection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccounts {
    pub id: String,
    pub status: String,
    pub agreements: Option<String>,
    pub accounts: Vec<String>,
    pub reference: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionAmount {
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DebtorAccount {
    pub iban: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreditorAccount {
    pub iban: Option<String>,
    pub bban: Option<String>,
    pub pan: Option<String>,
    pub masked_pan: Option<String>,
    pub msisdn: Option<String>,
    pub currency: Option<String>,
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
    pub booking_date: Option<String>,
    pub booking_date_time: Option<String>,
    pub value_date: Option<String>,
    pub value_date_time: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub bank_transaction_code: Option<String>,
    pub balance_after_minor: Option<i32>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoCardlessConfig {
    pub secret_id: String,
    pub secret_key: String,
}

#[derive(Debug)]
pub struct GoCardless {
    pub base_url: String,
    pub secret_id: String,
    pub secret_key: String,
    pub access_token: Option<AccessToken>,
}
