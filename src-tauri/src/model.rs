use crate::schema::{accounts, providers, tags, transaction_tags, transactions};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = providers)]
pub struct Provider {
    pub id: i32,
    pub title: String,
    pub secret_id: Option<String>,
    pub secret_key: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = providers)]
pub struct NewProvider {
    pub title: String,
    pub secret_id: Option<String>,
    pub secret_key: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BankInfo {
    pub id: String,
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BankConnectionInfo {
    pub id: String,
    pub link: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Provider, foreign_key = provider_id))]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub provider_id: i32,
    pub title: String,
    pub institution_id: Option<String>,
    pub bank_connection_id: Option<String>,
    pub account_id: Option<String>,
    pub iban: Option<String>,
}

#[typeshare]
#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub title: String,
    pub provider_id: i32,
    pub institution_id: String,
    pub bank_connection_id: String,
    pub account_id: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub title: String,
    pub debitor_name: Option<String>,
    pub debitor_iban: Option<String>,
    pub creditor_name: Option<String>,
    pub creditor_iban: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub date: String,
    pub remittance_information: Option<String>,
    pub account_id: i32,
}

#[typeshare]
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub title: String,
    pub debitor_name: Option<String>,
    pub debitor_iban: Option<String>,
    pub creditor_name: Option<String>,
    pub creditor_iban: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub date: String,
    pub remittance_information: Option<String>,
    pub account_id: i32,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}

#[typeshare]
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Associations)]
#[diesel(belongs_to(Transaction, foreign_key = transaction_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
#[diesel(table_name = transaction_tags)]
pub struct TransactionTag {
    pub transaction_id: i32,
    pub tag_id: i32,
}
