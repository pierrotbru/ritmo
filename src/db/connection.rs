use sqlx::{migrate, migrate::Migrator, query};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous, SqlitePool};
use std::{path::PathBuf, fs, str::FromStr};
use crate::errors::RitmoErr;
use crate::db::verify_path::verify_path;

static MIGRATOR: Migrator = migrate!();

pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    let db_path = verify_path(path, create)?;

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