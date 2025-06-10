//use sqlx::query;
//use sqlx::{sqlite::SqlitePool, migrate::Migrator, migrate};
//use std::path::PathBuf;
//use std::fs;
//
//// Includi le migrazioni compilate nel binario
//static MIGRATOR: Migrator = migrate!();
//
//use crate::errors::RitmoErr;
//use crate::db::verify_path::verify_path;
//
//
//
//pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
//    let db_path = verify_path(path, create)?;
//
//    if create {
//        if !db_path.exists() {
//            fs::File::create(db_path.clone()).map_err(|e| RitmoErr::IoError(e.to_string()))?;
//        }
//    }
//
//    let database_url = format!("sqlite:///{}", db_path.to_string_lossy());
//
//    let pool = SqlitePool::connect(&database_url).await.map_err(RitmoErr::SqlxError)?;
//
//    // Abilita le chiavi esterne
//    query("PRAGMA foreign_keys = ON;")
//        .execute(&pool)
//        .await
//        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
//
//    if create {
//        MIGRATOR.run(&pool).await.map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
//    }
//
//    Ok(pool)
//}


use sqlx::migrate;
use sqlx::migrate::Migrator;
use crate::errors::RitmoErr;
use crate::db::verify_path::verify_path; // Assicurati che questo percorso sia corretto

// Importa i tipi specifici di Sqlite per le opzioni di connessione
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;
use sqlx::query; // Già usato per PRAGMA foreign_keys
use std::path::PathBuf;
use std::fs;
use std::str::FromStr; // Necessario per SqliteConnectOptions::from_str

// Il MIGRATOR rimane qui
static MIGRATOR: Migrator = migrate!();

pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    let db_path = verify_path(path, create)?;

    // Se stiamo creando il DB per la prima volta, assicurati che il file esista.
    // Il `SqliteConnectOptions::create_if_missing(true)` potrebbe farlo anche.
    if create {
        if !db_path.exists() {
            fs::File::create(db_path.clone()).map_err(|e| RitmoErr::IoError(e.to_string()))?;
        }
    }

    let database_url = format!("sqlite:///{}", db_path.to_string_lossy());

    let mut options = SqliteConnectOptions::from_str(&database_url)
        .map_err(|e| RitmoErr::SqlxError(e))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    options = options
        .pragma("cache_size", "-64000")
        .pragma("temp_store", "MEMORY");

    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(RitmoErr::SqlxError)?;

    query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    if create {
        MIGRATOR.run(&pool).await.map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
    }

    query("ANALYZE;")
        .execute(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
    query("PRAGMA optimize;")
        .execute(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    Ok(pool)
}