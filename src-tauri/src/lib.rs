#![allow(clippy::redundant_field_names)]
#![allow(clippy::needless_lifetimes)]

pub mod banking;
mod commands;
mod database;
mod model;
mod schema;

use commands::*;
use database::DatabaseState;
use tauri::Manager;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview))
                .build(),
        );
    }

    builder = builder.plugin(tauri_plugin_shell::init());
    builder = builder.plugin(tauri_plugin_fs::init());

    builder = builder.setup(|app| {
        let path_database = app.path().app_data_dir()?.join("database.sqlite");
        let database_state = DatabaseState::new(path_database.clone());
        database_state.run_migrations();
        app.manage(Mutex::new(database_state));
        Ok(())
    });

    // builder = builder.manage();

    builder = builder.invoke_handler(tauri::generate_handler![
        list_possible_banking_providers,
        get_banking_providers,
        add_banking_provider,
        get_banks_by_country_handler,
        connect_bank_account_phase_1,
        connect_bank_account_phase_2,
        disconnect_bank_account,
        get_banking_accounts,
        get_transactions_handler,
        get_transactions
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
