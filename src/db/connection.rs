use sqlx::query;
use sqlx::{sqlite::SqlitePool, migrate::Migrator, migrate};
use std::path::PathBuf;
use std::fs;

// Includi le migrazioni compilate nel binario
static MIGRATOR: Migrator = migrate!();

use crate::errors::RitmoErr;
use crate::db::verify_path::verify_path;



pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    let db_path = verify_path(path, create)?;

    if create {
        if !db_path.exists() {
            fs::File::create(db_path.clone()).map_err(|e| RitmoErr::IoError(e.to_string()))?;
        }
    }

    let database_url = format!("sqlite:///{}", db_path.to_string_lossy());

    let pool = SqlitePool::connect(&database_url).await.map_err(RitmoErr::SqlxError)?;

    // Abilita le chiavi esterne
    query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    if create {
        MIGRATOR.run(&pool).await.map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
    }

    Ok(pool)
}
