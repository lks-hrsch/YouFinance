use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};
use diesel_migrations::{
    embed_migrations,
    EmbeddedMigrations,
    MigrationHarness,
};
use tauri_plugin_log::log::{
    debug,
    info,
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Default)]
pub struct DatabaseState {
    path: PathBuf,
}

impl DatabaseState {
    pub fn new(path: PathBuf) -> Self {
        if !path.exists() {
            let db_dir = Path::new(&path).parent().unwrap();

            if !db_dir.exists() {
                fs::create_dir_all(db_dir).unwrap();
            }

            fs::File::create(&path).unwrap();
        }

        debug!("database path: {:?}", path);
        Self { path }
    }

    pub fn run_migrations(&self) {
        info!("Running database migrations...");
        self.connection().run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn connection(&self) -> SqliteConnection {
        SqliteConnection::establish(self.path.to_str().unwrap()).unwrap()
    }
}
