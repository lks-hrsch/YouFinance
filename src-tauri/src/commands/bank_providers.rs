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
    banking::providers::BankingProviders,
    database::DatabaseState,
    model::*,
};

#[tauri::command]
pub fn list_possible_banking_providers() -> Vec<String> {
    debug!("commands::bank_providers::list_possible_banking_providers");
    BankingProviders::list_providers()
}

#[tauri::command]
pub async fn get_banking_providers(database_state: State<'_, Mutex<DatabaseState>>) -> Result<Vec<Provider>, String> {
    debug!("commands::bank_providers::get_banking_providers");
    use crate::schema::providers::dsl::*;

    let connection = &mut database_state.lock().await.connection();
    let p: Vec<Provider> = providers
        .select(Provider::as_select())
        .load(connection)
        .expect("error loading providers");

    Ok(p)
}

#[tauri::command]
pub async fn add_banking_provider(
    database_state: State<'_, Mutex<DatabaseState>>,
    name: String,
    sid: Option<String>,
    skey: Option<String>,
) -> Result<(), String> {
    debug!("commands::bank_providers::add_banking_provider");
    use crate::schema::providers::dsl::*;

    let connection = &mut database_state.lock().await.connection();
    let new_provider = NewProvider {
        title: name,
        secret_id: sid,
        secret_key: skey,
    };
    diesel::insert_into(providers::table())
        .values(&new_provider)
        .execute(connection)
        .map_err(|e| format!("Error saving new provider: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_banking_provider(
    database_state: State<'_, Mutex<DatabaseState>>,
    provider_id: i32,
) -> Result<(), String> {
    debug!("commands::bank_providers::delete_banking_provider ID: {}", provider_id);
    use crate::schema::providers::dsl::*;

    let mut connection = database_state.lock().await.connection();
    use crate::schema::{
        accounts::dsl as accounts_dsl,
    };

    // First, find all accounts for this provider to delete them using the shared logic
    let provider_accounts: Vec<i32> = accounts_dsl::accounts
        .filter(accounts_dsl::provider_id.eq(provider_id))
        .select(accounts_dsl::id)
        .load::<i32>(&mut connection)
        .map_err(|e| format!("Error finding accounts for provider: {}", e))?;

    // Delete each account using the helper (which also deletes transactions and tags)
    for account_id in provider_accounts {
        crate::commands::bank_accounts::delete_bank_account_internal(&mut connection, account_id)?;
    }

    diesel::delete(providers.filter(id.eq(provider_id)))
        .execute(&mut connection)
        .map_err(|e| format!("Error deleting provider: {}", e))?;

    Ok(())
}
