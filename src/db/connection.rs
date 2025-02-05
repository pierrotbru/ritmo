// src/db/connection.rs
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use sea_orm_migration::prelude::*;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;

use crate::errors::RitmoErr;
use crate::db::migration::Migrator;
use crate::db::verify_path::verify_path;

pub async fn establish_connection(path: &PathBuf, create: bool) -> Result<DatabaseConnection, RitmoErr> {
    // call verify_path to check if the db_path is valid
    let db_path = verify_path(path, create)?;

    if create {
        fs::File::create(db_path.clone())
            .map_err(|e| RitmoErr::DatabaseCreationFailed(
                format!("Failed to create database file at {}: {}", db_path.display(), e)
            ))?;   
    }

    // Construct the database URL with explicit SQLite driver
    let database_url = format!("sqlite:///{}", db_path.to_string_lossy());
    
    // Configure connection options with pooling
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
       .min_connections(5)
       .connect_timeout(Duration::from_secs(30))
       .idle_timeout(Duration::from_secs(300))
       .sqlx_logging(true);
    
    // Attempt to connect to the database with pooling
    let connection = Database::connect(opt)
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(
            format!("Failed to connect to database at {}: Connection details: {}", 
                db_path.display(), 
                e.to_string()
            )
        ))?;

    // Run migrations if create is true
    if create {
        Migrator::up(&connection, None)
            .await
            .map_err(|e| RitmoErr::DatabaseMigrationFailed(
                format!("Failed to run migrations: {}", e)
            ))?;
    }

    Ok(connection)
}