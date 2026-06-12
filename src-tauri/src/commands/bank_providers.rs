use diesel::{
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
        .map_err(|e| format!("Error loading providers: {}", e))?;

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
    use crate::schema::providers;

    let connection = &mut database_state.lock().await.connection();
    let provider_config_json = serde_json::json!({
        "secret_id": sid,
        "secret_key": skey,
    })
    .to_string();
    let new_provider = NewProvider {
        name,
        config_json: provider_config_json,
    };
    diesel::insert_into(providers::table)
        .values(&new_provider)
        .execute(connection)
        .map_err(|e| format!("Error saving new provider: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_banking_provider(
    database_state: State<'_, Mutex<DatabaseState>>,
    provider_id: i32,
    sid: String,
    skey: String,
) -> Result<(), String> {
    debug!("commands::bank_providers::update_banking_provider ID: {}", provider_id);
    use crate::schema::providers::dsl::*;

    let config = serde_json::json!({ "secret_id": sid, "secret_key": skey }).to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let connection = &mut database_state.lock().await.connection();

    diesel::update(providers.filter(id.eq(provider_id)))
        .set((config_json.eq(config), updated_at.eq(now)))
        .execute(connection)
        .map_err(|e| format!("Error updating provider: {}", e))?;

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
        bank_account_providers::dsl as bap_dsl,
    };

    // First, find all bank_account_ids for this provider to delete them using the shared logic
    let provider_accounts: Vec<i32> = bap_dsl::bank_account_providers
        .filter(bap_dsl::provider_id.eq(provider_id))
        .select(bap_dsl::bank_account_id)
        .load::<i32>(&mut connection)
        .map_err(|e| format!("Error finding accounts for provider: {}", e))?;

    // Delete each account using the helper (which also deletes transactions and tags)
    for bank_account_id in provider_accounts {
        crate::commands::bank_accounts::delete_bank_account_internal(&mut connection, bank_account_id)?;
    }

    diesel::delete(providers.filter(id.eq(provider_id)))
        .execute(&mut connection)
        .map_err(|e| format!("Error deleting provider: {}", e))?;

    Ok(())
}
