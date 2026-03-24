use diesel::{
    associations::HasTable,
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
    OptionalExtension,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::log::debug;
use tokio::sync::Mutex;

use crate::{
    banking::{
        apierror::ApiError,
        providers::BankingProviders,
        trait_banking_api::BankingApi,
        utils::normalize_date,
    },
    database::DatabaseState,
    model::*,
};

async fn sync_accounts_internal(
    app_handle: AppHandle,
    database_state: &Mutex<DatabaseState>,
    accounts_to_sync: Vec<Account>,
) -> Result<(), String> {
    use crate::schema::{
        providers::dsl as providers_dsl,
        transactions::dsl as transactions_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

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
        let amount: f64 = old_trans.transaction_amount.amount.parse().unwrap_or(0.0);
        let date = old_trans.booking_date.clone().unwrap_or("".into());

        NewTransaction {
            title: "".into(),
            debitor_name: old_trans.debtor_name.clone(),
            debitor_iban: debitor_iban,
            debitor_bic: None,
            creditor_name: old_trans.creditor_name.clone(),
            creditor_iban: creditor_iban,
            creditor_bic: None,
            amount: amount,
            currency: old_trans.transaction_amount.clone().currency,
            date: normalize_date(&date),
            remittance_information: old_trans.remittance_information_unstructured.clone(),
            account_id,
        }
    }

    let app_data_dir = app_handle.path().app_data_dir().map_err(|e: tauri::Error| e.to_string())?;

    for account in accounts_to_sync {
        let provider_data: Provider = providers_dsl::providers
            .filter(providers_dsl::id.eq(account.provider_id))
            .first::<Provider>(connection)
            .map_err(|e| format!("Failed to load provider for account {}: {}", account.id, e))?;

        let provider_enum = BankingProviders::from_string(&provider_data.title)
            .ok_or_else(|| format!("Invalid provider: {}", provider_data.title))?;
        
        let provider_instance = provider_enum.connect_provider(connection, app_data_dir.clone()).await?;

        let account_id_str = account.account_id.clone().unwrap_or_default();
        let account_transactions = provider_instance
            .get_account_transactions(&account_id_str)
            .await
            .map_err(|e: ApiError| format!("Failed to fetch transactions from provider for account {}: {}", account.id, e))?;

        let transactions_to_save: Vec<NewTransaction> = account_transactions
            .transactions
            .booked
            .iter()
            .map(|elem| transform_transaction(elem, account.id))
            .collect();

        for transaction in transactions_to_save {
            // Robust deduplication check because SQLite UNIQUE constraints fail with NULL values
            let mut query = transactions_dsl::transactions
                .filter(transactions_dsl::date.eq(&transaction.date))
                .filter(transactions_dsl::amount.eq(transaction.amount))
                .filter(transactions_dsl::account_id.eq(transaction.account_id))
                .into_boxed();

            if let Some(ref d_iban) = transaction.debitor_iban {
                query = query.filter(transactions_dsl::debitor_iban.eq(d_iban));
            } else {
                query = query.filter(transactions_dsl::debitor_iban.is_null());
            }

            if let Some(ref c_iban) = transaction.creditor_iban {
                query = query.filter(transactions_dsl::creditor_iban.eq(c_iban));
            } else {
                query = query.filter(transactions_dsl::creditor_iban.is_null());
            }

            if let Some(ref rem) = transaction.remittance_information {
                query = query.filter(transactions_dsl::remittance_information.eq(rem));
            } else {
                query = query.filter(transactions_dsl::remittance_information.is_null());
            }

            let exists = query
                .select(Transaction::as_select())
                .first::<Transaction>(connection)
                .optional()
                .map_err(|e| e.to_string())?
                .is_some();

            if !exists {
                diesel::insert_into(transactions_dsl::transactions::table())
                    .values(&transaction)
                    .on_conflict_do_nothing()
                    .execute(connection)
                    .map_err(|e| format!("Failed to save transaction to database: {}", e))?;
            }
        }
    }

    Ok(())
}



#[tauri::command]
pub async fn sync_all_accounts(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_all_accounts");
    use crate::schema::accounts::dsl::*;

    let all_accounts: Vec<Account>;
    {
        let connection = &mut database_state.lock().await.connection();
        all_accounts = accounts
            .select(Account::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load accounts: {}", e))?;
    }

    sync_accounts_internal(app_handle, &database_state, all_accounts).await
}

#[tauri::command]
pub async fn sync_provider_accounts(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
    p_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_provider_accounts: {}", p_id);
    use crate::schema::accounts::dsl::*;

    let filtered_accounts: Vec<Account>;
    {
        let connection = &mut database_state.lock().await.connection();
        filtered_accounts = accounts
            .filter(provider_id.eq(p_id))
            .select(Account::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load accounts for provider {}: {}", p_id, e))?;
    }

    sync_accounts_internal(app_handle, &database_state, filtered_accounts).await
}

#[tauri::command]
pub async fn sync_account(
    app_handle: AppHandle,
    database_state: State<'_, Mutex<DatabaseState>>,
    target_account_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_account_transactions::sync_account: {}", target_account_id);
    use crate::schema::accounts::dsl::*;

    let filtered_accounts: Vec<Account>;
    {
        let connection = &mut database_state.lock().await.connection();
        filtered_accounts = accounts
            .filter(id.eq(target_account_id))
            .select(Account::as_select())
            .load(connection)
            .map_err(|e| format!("Failed to load account {}: {}", target_account_id, e))?;
    }

    sync_accounts_internal(app_handle, &database_state, filtered_accounts).await
}



#[tauri::command]
pub async fn get_transactions(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<Transaction>, String> {
    debug!("commands::bank_account_transactions::get_transactions");
    use crate::schema::transactions::dsl::*;

    let connection = &mut database_state.lock().await.connection();

    let all_transactions: Vec<Transaction> = transactions
        .select(Transaction::as_select())
        .order_by(date.desc())
        .load(connection)
        .map_err(|e| format!("Failed to load transactions: {}", e))?;

    Ok(all_transactions)
}
