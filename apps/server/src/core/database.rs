use std::sync::OnceLock;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static URI: OnceLock<String> = OnceLock::new();

pub async fn init_db() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let db_path = exe_dir.join("data.db");

    let uri = format!("sqlite://{}?mode=rwc", db_path.to_str().unwrap());

    let mut connection = diesel::sqlite::SqliteConnection::establish(&uri).unwrap();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    URI.set(uri).expect("Error setting up global database URI");
}

pub fn create_connection() -> SqliteConnection {
    SqliteConnection::establish(URI.get().unwrap().as_str()).expect("Error connecting to database")
}
