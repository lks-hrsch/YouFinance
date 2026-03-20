use diesel::{
    associations::HasTable,
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
};
use tauri::State;
use tauri_plugin_log::log::debug;
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
pub async fn get_banks_by_country_handler<'a>(
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    country: String,
) -> Result<Vec<BankInfo>, String> {
    debug!("commands::bank_accounts::get_banks_by_country_handler");
    let provider = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let gocardless = provider.connect_provider(connection).await?;

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
pub async fn connect_bank_account_phase_1<'a>(
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    institution_id: String,
) -> Result<BankConnectionInfo, String> {
    debug!("commands::bank_accounts::connect_bank_account_phase_1");
    let provider = BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let gocardless = provider.connect_provider(connection).await?;

    let connect_bank_result = gocardless
        .connect_bank("http://localhost", &institution_id)
        .await
        .map_err(|e| e.to_string())?;

    let bank_connection_info = BankConnectionInfo {
        id: connect_bank_result.id.unwrap(),
        link: connect_bank_result.link.unwrap(),
    };

    Ok(bank_connection_info)
}

#[tauri::command]
pub async fn connect_bank_account_phase_2<'a>(
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
    let gocardless = provider.connect_provider(connection).await?;

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
            .map_err(|e| format!("Error saving new account: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn disconnect_bank_account<'a>(
    database_state: State<'a, Mutex<DatabaseState>>,
    provider_title: String,
    bank_connection_id: String,
) -> Result<(), String> {
    debug!("commands::bank_accounts::disconnect_bank_account");
    let provider = crate::banking::providers::BankingProviders::from_string(&provider_title)
        .ok_or_else(|| format!("Invalid provider: {}", provider_title))?;
    let connection = &mut database_state.lock().await.connection();
    let gocardless = provider.connect_provider(connection).await?;

    gocardless
        .disconnect_bank(&bank_connection_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_banking_accounts(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<Account>, String> {
    debug!("commands::bank_accounts::get_banking_accounts");
    use crate::schema::accounts::dsl as accounts_dsl;

    let connection = &mut database_state.lock().await.connection();

    let accounts: Vec<Account> = accounts_dsl::accounts
        .select(Account::as_select())
        .load(connection)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(accounts)
}
