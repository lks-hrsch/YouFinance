use diesel::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use typeshare::typeshare;

use crate::schema::{
    bank_account_providers,
    bank_accounts,
    providers,
    tags,
    transaction_providers,
    transaction_tags,
    transactions,
};

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewProvider {
    pub name: String,
    pub config_json: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BankInfo {
    pub id: String,
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct BankConnectionInfo {
    pub id: String,
    pub link: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = bank_accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BankAccount {
    pub id: i32,
    pub name: String,
    pub iban: Option<String>,
    pub bic: Option<String>,
    pub owner_name: Option<String>,
    pub currency_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = bank_accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewBankAccount {
    pub name: String,
    pub iban: Option<String>,
    pub bic: Option<String>,
    pub owner_name: Option<String>,
    pub currency_code: Option<String>,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(BankAccount, foreign_key = bank_account_id))]
#[diesel(belongs_to(Provider, foreign_key = provider_id))]
#[diesel(table_name = bank_account_providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BankAccountProvider {
    pub id: i32,
    pub bank_account_id: i32,
    pub provider_id: i32,
    pub bank_connection_id: String,
    pub institution_id: Option<String>,
    pub external_account_id: Option<String>,
    pub last_synced_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = bank_account_providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewBankAccountProvider {
    pub bank_account_id: i32,
    pub provider_id: i32,
    pub bank_connection_id: String,
    pub institution_id: Option<String>,
    pub external_account_id: Option<String>,
    pub last_synced_at: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccountWithProvider {
    pub bank_account: BankAccount,
    pub bank_account_provider: BankAccountProvider,
    pub provider_name: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(BankAccount, foreign_key = bank_account_id))]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub booking_text: String,
    pub debtor_name: Option<String>,
    pub debtor_iban: Option<String>,
    pub debtor_bic: Option<String>,
    pub creditor_name: Option<String>,
    pub creditor_iban: Option<String>,
    pub creditor_bic: Option<String>,
    pub amount_minor: i32,
    pub currency_code: String,
    pub booking_date: String,
    pub value_date: Option<String>,
    pub balance_after_minor: Option<i32>,
    pub mandate_reference: Option<String>,
    pub remittance_information: Option<String>,
    pub bank_account_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTransaction {
    pub booking_text: String,
    pub debtor_name: Option<String>,
    pub debtor_iban: Option<String>,
    pub debtor_bic: Option<String>,
    pub creditor_name: Option<String>,
    pub creditor_iban: Option<String>,
    pub creditor_bic: Option<String>,
    pub amount_minor: i32,
    pub currency_code: String,
    pub booking_date: String,
    pub value_date: Option<String>,
    pub balance_after_minor: Option<i32>,
    pub mandate_reference: Option<String>,
    pub remittance_information: Option<String>,
    pub bank_account_id: i32,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTag {
    pub name: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Associations)]
#[diesel(belongs_to(Transaction, foreign_key = transaction_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
#[diesel(table_name = transaction_tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionTag {
    pub transaction_id: i32,
    pub tag_id: i32,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = transaction_providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionProvider {
    pub transaction_id: i32,
    pub provider_id: i32,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

#[typeshare]
#[derive(Debug, Insertable)]
#[diesel(table_name = transaction_providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTransactionProvider {
    pub transaction_id: i32,
    pub provider_id: i32,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionWithProviders {
    pub transaction: Transaction,
    pub providers: Vec<String>,
}
