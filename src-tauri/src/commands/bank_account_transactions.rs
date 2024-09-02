use diesel::{
    associations::HasTable,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
};
use log::debug;
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    banking::{
        providers::BankingProviders,
        trait_banking_api::BankingApi,
    },
    database::DatabaseState,
    model::*,
};

#[tauri::command]
pub async fn get_transactions_handler<'a>(database_state: State<'a, Mutex<DatabaseState>>) -> Result<(), String> {
    debug!("commands::bank_account_transactions::get_transactions_handler");
    use crate::schema::{
        accounts::dsl as accounts_dsl,
        providers::dsl as providers_dsl,
        transactions::dsl as transactions_dsl,
    };

    let connection = &mut database_state.lock().await.connection();
    let provider: Provider = providers_dsl::providers
        .first::<Provider>(connection)
        .map_err(|e| e.to_string())?;

    let provider = BankingProviders::from_string(&provider.title).unwrap();
    let gocardless = provider.connect_provider(connection).await?;

    let accounts: Vec<Account> = accounts_dsl::accounts
        .select(Account::as_select())
        .load(connection)
        .expect("error loading accounts");

    fn transform_transaction(
        old_trans: &crate::banking::providers::gocardless::structs::Transaction,
        account_id: i32,
    ) -> NewTransaction {
        let debitor_iban = old_trans.debtor_account.as_ref().map(|account| account.iban.clone());
        let creditor_iban = old_trans
            .creditor_account
            .as_ref()
            .map(|account| account.iban.clone())
            .unwrap_or(None);
        let amount: f64 = old_trans.transaction_amount.amount.parse().unwrap();
        let date = old_trans.booking_date.clone().unwrap_or("".into());

        NewTransaction {
            title: "".into(),
            debitor_name: old_trans.debtor_name.clone(),
            debitor_iban: debitor_iban,
            creditor_name: old_trans.creditor_name.clone(),
            creditor_iban: creditor_iban,
            amount: amount,
            currency: old_trans.transaction_amount.clone().currency,
            date: date,
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

    for transaction in &transactions {
        diesel::insert_into(transactions_dsl::transactions::table())
            .values(transaction)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Error inserting transactions");
    }

    Ok(())
}

#[tauri::command]
pub async fn get_transactions(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<Transaction>, String> {
    debug!("commands::bank_account_transactions::get_transactions");
    use crate::schema::transactions::dsl as transaction_dsl;

    let connection = &mut database_state.lock().await.connection();

    let mut transactions: Vec<Transaction> = transaction_dsl::transactions
        .select(Transaction::as_select())
        .load(connection)
        .expect("error loading transactions");

    // Sort the transactions by date
    transactions.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(transactions)
}
