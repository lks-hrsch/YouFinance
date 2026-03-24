pub mod banking;
mod commands;
mod database;
pub mod model;
pub mod schema;

use tauri::Manager;
use tauri_plugin_log::log::info;
use tokio::sync::Mutex;

use crate::{
    commands::*,
    database::DatabaseState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_transactions_handler,
            get_transactions,
            get_banks_by_country_handler,
            connect_bank_account_phase_1,
            connect_bank_account_phase_2,
            disconnect_bank_account,
            get_banking_accounts,
            list_possible_banking_providers,
            get_banking_providers,
            add_banking_provider,
        ])
        .setup(|app| {
            let path_database = app.path().app_data_dir()?.join("database.sqlite");
            info!("Database initialized at {path_database:?}");

            let database_state = DatabaseState::new(path_database.clone());
            database_state.run_migrations();
            app.manage(Mutex::new(database_state));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
