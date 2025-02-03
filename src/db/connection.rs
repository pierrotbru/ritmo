// src/db/connection.rs
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use std::path::PathBuf;
use std::fs;
use crate::errors::RitmoErr;
use crate::db::migration::Migrator;

pub async fn establish_connection(db_path: &PathBuf, create: bool) -> Result<DatabaseConnection, RitmoErr> {
    // Create an empty file if create is true
    if create {
        fs::File::create(db_path)
            .map_err(|e| RitmoErr::DatabaseCreationFailed(
                format!("Failed to create database file at {}: {}", db_path.display(), e)
            ))?;
    }

    // Construct the database URL
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    // Attempt to connect to the database
    let connection = Database::connect(&database_url)
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(
            format!("Failed to connect to database at {}: {}", db_path.display(), e)
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