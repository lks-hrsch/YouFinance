use diesel::{
    associations::HasTable,
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
        .expect("error saving new provider");

    Ok(())
}
