// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod banking;
mod database;
mod model;
mod schema;

use crate::banking::trait_banking_api::BankingApi;
use banking::providers::BankingProviders;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use model::*;

#[tauri::command]
fn list_possible_banking_providers() -> Vec<String> {
    BankingProviders::list_providers()
}

#[tauri::command]
fn get_banking_providers() -> Vec<Provider> {
    use schema::providers::dsl::*;

    let connection = &mut database::establish_db_connection();
    let p: Vec<Provider> = providers
        .select(Provider::as_select())
        .load(connection)
        .expect("error loading providers");
    p
}

#[tauri::command]
fn add_banking_provider(name: String, sid: Option<String>, skey: Option<String>) {
    use schema::providers::dsl::*;

    let connection = &mut database::establish_db_connection();
    let new_provider = NewProvider {
        title: name,
        secret_id: sid,
        secret_key: skey,
    };

    diesel::insert_into(providers::table())
        .values(&new_provider)
        .execute(connection)
        .expect("error saving new provider");
}

#[tauri::command]
async fn get_banks_by_country_handler(provider_title: String, country: String) -> Result<Vec<BankInfo>, String> {
    let provider = BankingProviders::from_string(&provider_title).unwrap();
    let gocardless = provider.connect_provider().await.unwrap();

    let banks = gocardless
        .get_banks_by_country(&country)
        .await
        .map_err(|e| e.to_string())?;
    let bank_names = banks
        .into_iter()
        .map(|bank| BankInfo {
            id: bank.id,
            name: bank.name,
        })
        .collect();

    Ok(bank_names)
}

#[tauri::command]
async fn connect_bank_account_phase_1(
    provider_title: String,
    institution_id: String,
) -> Result<BankConnectionInfo, String> {
    let provider = BankingProviders::from_string(&provider_title).unwrap();
    let gocardless = provider.connect_provider().await.unwrap();

    let connect_bank_result = gocardless
        .connect_bank("http://localhost", &institution_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(BankConnectionInfo {
        id: connect_bank_result.id.unwrap(),
        link: connect_bank_result.link.unwrap(),
    })
}

#[tauri::command]
async fn connect_bank_account_phase_2(
    provider_title: String,
    institution_id: String,
    requisition_id: String,
) -> Result<(), String> {
    use schema::accounts::dsl as accounts_dsl;
    use schema::providers::dsl as providers_dsl;

    let provider = BankingProviders::from_string(&provider_title).unwrap();
    let gocardless = provider.connect_provider().await.unwrap();

    let connection = &mut database::establish_db_connection();
    let provider: Provider = providers_dsl::providers
        .filter(providers_dsl::title.eq(provider_title))
        .first::<Provider>(connection)
        .map_err(|e| e.to_string())?;

    let bank_connections = gocardless
        .get_bank_accounts(&requisition_id)
        .await
        .map_err(|e| e.to_string())?;

    for account_id in bank_connections.accounts {
        let new_account = NewAccount {
            title: provider.title.clone(),
            provider_id: provider.id,
            institution_id: institution_id.clone(),
            bank_connection_id: requisition_id.clone(),
            account_id: account_id,
        };
        diesel::insert_into(accounts_dsl::accounts::table())
            .values(&new_account)
            .execute(connection)
            .expect("error saving new account");
    }
    Ok(())
}

#[tauri::command]
fn get_banking_accounts() -> Vec<Account> {
    use schema::accounts::dsl as accounts_dsl;

    let connection = &mut database::establish_db_connection();
    let p: Vec<Account> = accounts_dsl::accounts
        .select(Account::as_select())
        .load(connection)
        .expect("error loading accounts");
    p
}

#[tauri::command]
async fn get_transactions_handler() -> Result<Vec<Transaction>, String> {
    use schema::accounts::dsl as accounts_dsl;
    use schema::providers::dsl as providers_dsl;

    let connection = &mut database::establish_db_connection();
    let provider: Provider = providers_dsl::providers
        .first::<Provider>(connection)
        .map_err(|e| e.to_string())?;

    let provider = BankingProviders::from_string(&provider.title).unwrap();
    let gocardless = provider.connect_provider().await.unwrap();

    let accounts: Vec<Account> = accounts_dsl::accounts
        .select(Account::as_select())
        .load(connection)
        .expect("error loading accounts");

    fn transform_transaction(
        old_trans: &banking::provider_gocardless_structs::Transaction,
        account_id: i32,
    ) -> Transaction {
        let debitor_iban = old_trans.debtor_account.as_ref().map(|account| account.iban.clone());
        let creditor_iban = old_trans
            .creditor_account
            .as_ref()
            .map(|account| account.iban.clone())
            .unwrap_or(None);
        let ammount: f32 = old_trans.transaction_amount.amount.parse().unwrap();

        Transaction {
            id: 0,
            title: "".to_string(),
            debitor_name: old_trans.debtor_name.clone(),
            debitor_iban: debitor_iban,
            creditor_name: old_trans.creditor_name.clone(),
            creditor_iban: creditor_iban,
            ammount: ammount,
            currency: old_trans.transaction_amount.clone().currency,
            date: old_trans.booking_date.clone(),
            remittance_information: old_trans.remittance_information_unstructured.clone(),
            account_id: account_id,
        }
    }

    let mut transactions = vec![];
    for account in accounts {
        let account_id = account.account_id.unwrap();
        let account_transactions = gocardless
            .get_account_transactions(&account_id)
            .await
            .map_err(|e| e.to_string())?;
        transactions.extend(
            account_transactions
                .transactions
                .booked
                .iter()
                .map(|elem| transform_transaction(elem, account.id)),
        );
    }

    // Sort the transactions by date
    transactions.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(transactions)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            database::init();
            Ok(())
        })
        // .manage()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            list_possible_banking_providers,
            get_banking_providers,
            add_banking_provider,
            get_banks_by_country_handler,
            connect_bank_account_phase_1,
            connect_bank_account_phase_2,
            get_banking_accounts,
            get_transactions_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
