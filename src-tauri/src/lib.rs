pub mod banking;
mod commands;
mod database;
pub mod model;
pub mod schema;

use tauri::{Manager, Emitter};
use tauri_plugin_log::log::{info, error};
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
            sync_all_accounts,
            sync_provider_accounts,
            sync_account,
            get_transactions,
            get_banks_by_country_handler,
            connect_bank_account_phase_1,
            connect_bank_account_phase_2,
            disconnect_bank_account,
            get_banking_accounts,
            list_possible_banking_providers,
            get_banking_providers,
            add_banking_provider,
            delete_banking_provider,
            add_local_csv_account,
            delete_bank_account,
            open_account_directory,
        ])
        .setup(|app| {
            let path_database = app.path().app_data_dir()?.join("database.sqlite");
            info!("Database initialized at {path_database:?}");
 
            let database_state = DatabaseState::new(path_database.clone());
            database_state.run_migrations();
            app.manage(Mutex::new(database_state));

            // Start a background listener for GoCardless redirects
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let listener = match tokio::net::TcpListener::bind("127.0.0.1:8888").await {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Failed to bind GoCardless redirect listener: {}", e);
                        return;
                    }
                };
                info!("GoCardless redirect listener started on 127.0.0.1:8888");

                loop {
                    if let Ok((mut socket, _)) = listener.accept().await {
                        let handle_clone = handle.clone();
                        tokio::spawn(async move {
                            let mut buffer = [0; 1024];
                            if let Ok(n) = tokio::io::AsyncReadExt::read(&mut socket, &mut buffer).await {
                                let request = String::from_utf8_lossy(&buffer[..n]);
                                if let Some(start) = request.find("ref=") {
                                    let rest = &request[start + 4..];
                                    let end = rest.find(' ').unwrap_or(rest.len());
                                    let ref_id = &rest[..end];
                                    let ref_id = ref_id.split('&').next().unwrap_or(ref_id);

                                    info!("Captured GoCardless redirect with ref: {}", ref_id);
                                    let _ = handle_clone.emit("gocardless-redirect", ref_id);

                                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Success!</h1><p>You can close this tab and return to the app.</p></body></html>";
                                    let _ = tokio::io::AsyncWriteExt::write_all(&mut socket, response.as_bytes()).await;
                                }
                            }
                        });
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
