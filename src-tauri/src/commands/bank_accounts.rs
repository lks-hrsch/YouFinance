use diesel::{
    associations::HasTable,
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::log::{debug, info};
use tokio::sync::Mutex;
use std::fs;

use crate::{
    banking::{
        apierror::ApiError,
        providers::BankingProviders,
        trait_banking_api::BankingApi,
    },
    database::DatabaseState,
    model::*,
};

#[tauri::command]
pub async fn get_banks_by_country_handler<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    country: String,
) -> Result<Vec<BankInfo>, String> {
    debug!("commands::bank_accounts::get_banks_by_country_handler");
    let provider = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let provider_instance = provider.connect_provider(connection, app_data_dir).await?;

    let banks = provider_instance
        .get_banks_by_country(&country)
        .await
        .map_err(|e: ApiError| e.to_string())?;
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
pub async fn connect_bank_account_phase_1<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    institution_id: String,
) -> Result<BankConnectionInfo, String> {
    debug!("commands::bank_accounts::connect_bank_account_phase_1");
    let provider = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let provider_instance = provider.connect_provider(connection, app_data_dir).await?;

    let connect_bank_result = provider_instance
        .connect_bank("http://localhost:8888", &institution_id)
        .await
        .map_err(|e: ApiError| e.to_string())?;

    let bank_connection_info = BankConnectionInfo {
        id: connect_bank_result.id.unwrap(),
        link: connect_bank_result.link.unwrap(),
    };

    Ok(bank_connection_info)
}

#[tauri::command]
pub async fn connect_bank_account_phase_2<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    institution_id: String,
    requisition_id: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::connect_bank_account_phase_2");
    use crate::schema::{
        accounts::dsl as accounts_dsl,
        providers::dsl as providers_dsl,
    };

    let provider = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let provider_instance = provider.connect_provider(connection, app_data_dir).await?;

    let provider: Provider = providers_dsl::providers
        .filter(providers_dsl::title.eq(provider_title))
        .first::<Provider>(connection)
        .map_err(|e| e.to_string())?;

    let bank_connections = provider_instance
        .get_bank_accounts(&requisition_id)
        .await
        .map_err(|e: ApiError| e.to_string())?;

    for account_id in bank_connections.accounts {
        let new_account = NewAccount {
            title: provider.title.clone(),
            provider_id: provider.id,
            institution_id: Some(institution_id.clone()),
            bank_connection_id: requisition_id.clone(),
            account_id: Some(account_id),
            iban: None,
        };
        diesel::insert_into(accounts_dsl::accounts::table())
            .values(&new_account)
            .execute(connection)
            .map_err(|e| format!("Error saving new account: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn disconnect_bank_account<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    bank_connection_id: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::disconnect_bank_account");
    let provider = crate::banking::providers::BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let provider_instance = provider.connect_provider(connection, app_data_dir).await?;

    provider_instance
        .disconnect_bank(&bank_connection_id)
        .await
        .map_err(|e: ApiError| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_banking_accounts(
    database_state: State<'_, Mutex<DatabaseState>>,
) -> Result<Vec<AccountWithProvider>, String> {
    debug!("commands::bank_accounts::get_banking_accounts");
    use crate::schema::{
        accounts::dsl as accounts_dsl,
        providers::dsl as providers_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    let results = accounts_dsl::accounts
        .inner_join(providers_dsl::providers)
        .select((Account::as_select(), providers_dsl::title))
        .load::<(Account, String)>(connection)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    let accounts_with_provider = results
        .into_iter()
        .map(|(account, provider_title)| AccountWithProvider {
            account,
            provider_title,
        })
        .collect();

    Ok(accounts_with_provider)
}

#[tauri::command]
pub async fn add_local_csv_account<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    bank_name: String,
    iban: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::add_local_csv_account");
    use crate::schema::{
        accounts::dsl as accounts_dsl,
        providers::dsl as providers_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    // Ensure LocalCSV provider exists
    let provider_title = BankingProviders::LocalCSV.to_string();
    let provider_result = providers_dsl::providers
        .filter(providers_dsl::title.eq(&provider_title))
        .first::<Provider>(connection);

    let provider = match provider_result {
        Ok(p) => p,
        Err(_) => {
            let new_provider = NewProvider {
                title: provider_title.clone(),
                secret_id: None,
                secret_key: None,
            };
            diesel::insert_into(providers_dsl::providers)
                .values(&new_provider)
                .execute(connection)
                .map_err(|e| format!("Error creating LocalCSV provider: {}", e))?;

            providers_dsl::providers
                .filter(providers_dsl::title.eq(&provider_title))
                .first::<Provider>(connection)
                .map_err(|e| format!("Error loading LocalCSV provider: {}", e))?
        }
    };

    // Create the directory structure: <app_data_dir>/<bank_name>/<iban>
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let account_path = app_data_dir.join(&bank_name).join(&iban);
    
    fs::create_dir_all(&account_path)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;
    
    info!("Created data directory at {:?}", account_path);

    // Insert the account into the database
    let new_account = NewAccount {
        title: bank_name.clone(),
        provider_id: provider.id,
        institution_id: Some(bank_name),
        bank_connection_id: "local".into(),
        account_id: Some(iban.clone()),
        iban: Some(iban),
    };

    diesel::insert_into(accounts_dsl::accounts::table())
        .values(&new_account)
        .execute(connection)
        .map_err(|e| format!("Error saving new account: {}", e))?;

    Ok(())
}

pub fn delete_bank_account_internal(
    connection: &mut diesel::sqlite::SqliteConnection,
    account_id: i32,
) -> Result<(), String> {
    use crate::schema::{
        accounts::dsl as accounts_dsl,
        transactions::dsl as transactions_dsl,
        transaction_tags::dsl as tt_dsl,
    };

    // First, find all transactions for this account to delete their tags
    let tx_ids: Vec<i32> = transactions_dsl::transactions
        .filter(transactions_dsl::account_id.eq(account_id))
        .select(transactions_dsl::id)
        .load::<i32>(connection)
        .map_err(|e| format!("Error finding transactions for account: {}", e))?;

    if !tx_ids.is_empty() {
        // Delete tags associated with these transactions
        diesel::delete(tt_dsl::transaction_tags.filter(tt_dsl::transaction_id.eq_any(tx_ids)))
            .execute(connection)
            .map_err(|e| format!("Error deleting transaction tags: {}", e))?;
    }

    // Delete all transactions associated with this account
    diesel::delete(transactions_dsl::transactions.filter(transactions_dsl::account_id.eq(account_id)))
        .execute(connection)
        .map_err(|e| format!("Error deleting transactions for account: {}", e))?;

    // Delete the account
    diesel::delete(accounts_dsl::accounts.filter(accounts_dsl::id.eq(account_id)))
        .execute(connection)
        .map_err(|e| format!("Error deleting account: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_bank_account<'a>(
    database_state: State<'a, Mutex<DatabaseState>>,
    account_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_accounts::delete_bank_account ID: {}", account_id);
    let mut connection = database_state.lock().await.connection();

    delete_bank_account_internal(&mut connection, account_id)
}

#[tauri::command]
pub async fn open_account_directory(
    app_handle: AppHandle,
    bank_name: String,
    iban: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::open_account_directory for {}/{}", bank_name, iban);
    use tauri_plugin_opener::OpenerExt;

    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let account_path = app_data_dir.join(&bank_name).join(&iban);

    if !account_path.exists() {
        return Err(format!("Directory does not exist: {:?}", account_path));
    }

    app_handle
        .opener()
        .open_path(account_path.to_string_lossy().to_string(), None::<String>)
        .map_err(|e| format!("Failed to open directory: {}", e))?;

    Ok(())
}
