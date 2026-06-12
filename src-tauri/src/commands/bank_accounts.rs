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
        id: connect_bank_result.id
            .ok_or_else(|| "Bank connection ID not provided by provider".to_string())?,
        link: connect_bank_result.link
            .ok_or_else(|| "Bank connection link not provided by provider".to_string())?,
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
        bank_accounts::dsl as bank_accounts_dsl,
        bank_account_providers::dsl as bap_dsl,
        providers::dsl as providers_dsl,
    };

    let provider_enum = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let provider_instance = provider_enum.connect_provider(connection, app_data_dir).await?;

    let provider: Provider = providers_dsl::providers
        .filter(providers_dsl::name.eq(&provider_title))
        .first::<Provider>(connection)
        .map_err(|e| e.to_string())?;

    let bank_connections = provider_instance
        .get_bank_accounts(&requisition_id)
        .await
        .map_err(|e: ApiError| e.to_string())?;

    for external_account_id in bank_connections.accounts {
        let new_bank_account = NewBankAccount {
            name: provider.name.clone(),
            iban: None,
            bic: None,
            owner_name: None,
            currency_code: None,
        };
        diesel::insert_into(bank_accounts_dsl::bank_accounts::table())
            .values(&new_bank_account)
            .execute(connection)
            .map_err(|e| format!("Error saving new bank account: {}", e))?;

        let bank_account: BankAccount = bank_accounts_dsl::bank_accounts
            .order(bank_accounts_dsl::id.desc())
            .first::<BankAccount>(connection)
            .map_err(|e| format!("Error retrieving inserted bank account: {}", e))?;

        let new_bap = NewBankAccountProvider {
            bank_account_id: bank_account.id,
            provider_id: provider.id,
            bank_connection_id: requisition_id.clone(),
            institution_id: Some(institution_id.clone()),
            external_account_id: Some(external_account_id),
            last_synced_at: None,
        };
        diesel::insert_into(bap_dsl::bank_account_providers::table())
            .values(&new_bap)
            .execute(connection)
            .map_err(|e| format!("Error saving new bank_account_provider: {}", e))?;
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
) -> Result<Vec<BankAccountWithProvider>, String> {
    debug!("commands::bank_accounts::get_banking_accounts");
    use crate::schema::{
        bank_accounts::dsl as bank_accounts_dsl,
        bank_account_providers::dsl as bap_dsl,
        providers::dsl as providers_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    let results = bank_accounts_dsl::bank_accounts
        .inner_join(bap_dsl::bank_account_providers.inner_join(providers_dsl::providers))
        .select((BankAccount::as_select(), BankAccountProvider::as_select(), providers_dsl::name))
        .load::<(BankAccount, BankAccountProvider, String)>(connection)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    let accounts_with_provider = results
        .into_iter()
        .map(|(bank_account, bank_account_provider, provider_name)| BankAccountWithProvider {
            bank_account,
            bank_account_provider,
            provider_name,
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
        bank_accounts::dsl as bank_accounts_dsl,
        bank_account_providers::dsl as bap_dsl,
        providers::dsl as providers_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    // Ensure LocalCSV provider exists
    let provider_name = BankingProviders::LocalCSV.to_string();
    let provider_result = providers_dsl::providers
        .filter(providers_dsl::name.eq(&provider_name))
        .first::<Provider>(connection);

    let provider = match provider_result {
        Ok(p) => p,
        Err(_) => {
            let new_provider = NewProvider {
                name: provider_name.clone(),
                config_json: "{}".to_string(),
            };
            diesel::insert_into(providers_dsl::providers)
                .values(&new_provider)
                .execute(connection)
                .map_err(|e| format!("Error creating LocalCSV provider: {}", e))?;

            providers_dsl::providers
                .filter(providers_dsl::name.eq(&provider_name))
                .first::<Provider>(connection)
                .map_err(|e| format!("Error loading LocalCSV provider: {}", e))?
        }
    };

    // Create the directory structure: <app_data_dir>/<bank_name>/<iban>
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let account_path = app_data_dir.join(&bank_name).join(&iban);

    fs::create_dir_all(&account_path)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    info!("Created bank account data directory");

    // Insert the bank account into the database
    let new_bank_account = NewBankAccount {
        name: bank_name.clone(),
        iban: Some(iban.clone()),
        bic: None,
        owner_name: None,
        currency_code: None,
    };

    diesel::insert_into(bank_accounts_dsl::bank_accounts::table())
        .values(&new_bank_account)
        .execute(connection)
        .map_err(|e| format!("Error saving new bank account: {}", e))?;

    let bank_account: BankAccount = bank_accounts_dsl::bank_accounts
        .order(bank_accounts_dsl::id.desc())
        .first::<BankAccount>(connection)
        .map_err(|e| format!("Error retrieving inserted bank account: {}", e))?;

    // Insert the bank_account_provider link
    let new_bap = NewBankAccountProvider {
        bank_account_id: bank_account.id,
        provider_id: provider.id,
        bank_connection_id: "local".to_string(),
        institution_id: Some(bank_name),
        external_account_id: Some(iban),
        last_synced_at: None,
    };

    diesel::insert_into(bap_dsl::bank_account_providers::table())
        .values(&new_bap)
        .execute(connection)
        .map_err(|e| format!("Error saving new bank_account_provider: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn add_local_mta_account<'a>(
    app_handle: AppHandle,
    database_state: State<'a, Mutex<DatabaseState>>,
    bank_name: String,
    iban: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::add_local_mta_account");
    use crate::schema::{
        bank_accounts::dsl as bank_accounts_dsl,
        bank_account_providers::dsl as bap_dsl,
        providers::dsl as providers_dsl,
    };

    let connection = &mut database_state.lock().await.connection();

    // Ensure LocalMTA provider exists
    let provider_name = BankingProviders::LocalMTA.to_string();
    let provider_result = providers_dsl::providers
        .filter(providers_dsl::name.eq(&provider_name))
        .first::<Provider>(connection);

    let provider = match provider_result {
        Ok(p) => p,
        Err(_) => {
            let new_provider = NewProvider {
                name: provider_name.clone(),
                config_json: "{}".to_string(),
            };
            diesel::insert_into(providers_dsl::providers)
                .values(&new_provider)
                .execute(connection)
                .map_err(|e| format!("Error creating LocalMTA provider: {}", e))?;

            providers_dsl::providers
                .filter(providers_dsl::name.eq(&provider_name))
                .first::<Provider>(connection)
                .map_err(|e| format!("Error loading LocalMTA provider: {}", e))?
        }
    };

    // Create the directory structure: <app_data_dir>/<bank_name>/<iban>
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let account_path = app_data_dir.join(&bank_name).join(&iban);

    fs::create_dir_all(&account_path)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    info!("Created data directory for bank account");

    // Insert the bank account into the database
    let new_bank_account = NewBankAccount {
        name: bank_name.clone(),
        iban: Some(iban.clone()),
        bic: None,
        owner_name: None,
        currency_code: None,
    };

    diesel::insert_into(bank_accounts_dsl::bank_accounts::table())
        .values(&new_bank_account)
        .execute(connection)
        .map_err(|e| format!("Error saving new bank account: {}", e))?;

    let bank_account: BankAccount = bank_accounts_dsl::bank_accounts
        .order(bank_accounts_dsl::id.desc())
        .first::<BankAccount>(connection)
        .map_err(|e| format!("Error retrieving inserted bank account: {}", e))?;

    // Insert the bank_account_provider link
    let new_bap = NewBankAccountProvider {
        bank_account_id: bank_account.id,
        provider_id: provider.id,
        bank_connection_id: "local".to_string(),
        institution_id: Some(bank_name),
        external_account_id: Some(iban),
        last_synced_at: None,
    };

    diesel::insert_into(bap_dsl::bank_account_providers::table())
        .values(&new_bap)
        .execute(connection)
        .map_err(|e| format!("Error saving new bank_account_provider: {}", e))?;

    Ok(())
}

pub fn delete_bank_account_internal(
    connection: &mut diesel::sqlite::SqliteConnection,
    bank_account_id: i32,
) -> Result<(), String> {
    use crate::schema::{
        bank_accounts::dsl as bank_accounts_dsl,
        bank_account_providers::dsl as bap_dsl,
        transactions::dsl as transactions_dsl,
        transaction_tags::dsl as tt_dsl,
        transaction_providers::dsl as tp_dsl,
    };

    // First, find all transactions for this account to delete their tags and providers
    let tx_ids: Vec<i32> = transactions_dsl::transactions
        .filter(transactions_dsl::bank_account_id.eq(bank_account_id))
        .select(transactions_dsl::id)
        .load::<i32>(connection)
        .map_err(|e| format!("Error finding transactions for account: {}", e))?;

    if !tx_ids.is_empty() {
        // Delete tags associated with these transactions
        diesel::delete(tt_dsl::transaction_tags.filter(tt_dsl::transaction_id.eq_any(&tx_ids)))
            .execute(connection)
            .map_err(|e| format!("Error deleting transaction tags: {}", e))?;

        // Delete providers associated with these transactions
        diesel::delete(tp_dsl::transaction_providers.filter(tp_dsl::transaction_id.eq_any(&tx_ids)))
            .execute(connection)
            .map_err(|e| format!("Error deleting transaction providers: {}", e))?;
    }

    // Delete all transactions associated with this account
    diesel::delete(transactions_dsl::transactions.filter(transactions_dsl::bank_account_id.eq(bank_account_id)))
        .execute(connection)
        .map_err(|e| format!("Error deleting transactions for account: {}", e))?;

    // Delete bank_account_provider links
    diesel::delete(bap_dsl::bank_account_providers.filter(bap_dsl::bank_account_id.eq(bank_account_id)))
        .execute(connection)
        .map_err(|e| format!("Error deleting bank_account_providers: {}", e))?;

    // Delete the bank account
    diesel::delete(bank_accounts_dsl::bank_accounts.filter(bank_accounts_dsl::id.eq(bank_account_id)))
        .execute(connection)
        .map_err(|e| format!("Error deleting bank account: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_bank_account<'a>(
    database_state: State<'a, Mutex<DatabaseState>>,
    account_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_accounts::delete_bank_account invoked");
    let mut connection = database_state.lock().await.connection();

    delete_bank_account_internal(&mut connection, account_id)
}

#[tauri::command]
pub async fn open_account_directory(
    app_handle: AppHandle,
    bank_name: String,
    iban: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::open_account_directory for bank: {}", bank_name);
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
