// src/db/connection.rs
use sqlx::SqlitePool;
use std::path::Path;
use csv::ReaderBuilder;
use sea_orm::Database;
use sea_orm::SqlxSqliteConnector;
use sea_orm_migration::prelude::*;
use std::path::PathBuf;
use std::fs;

use crate::errors::RitmoErr;
use crate::db::migration::Migrator;
use crate::db::verify_path::verify_path;

use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::Set;
use crate::db::entity::prelude::*;

/*
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

        // after migration try to copy all the iso codes and names
        seed_languages_names_table(&connection).await;
    }

    Ok(connection)
}
*/

async fn get_languages_names() -> Result<Vec<(String, String)>, DbErr> {
    let path = Path::new("./resources/iso-639-2.tab");

    // More robust path handling:
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            // Convert the io::Error to DbErr.  You might want a more specific DbErr variant.
            return Err(DbErr::Custom(format!("Error canonicalizing path: {}", e)));
        }
    };

    let mut reader = match ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false) // Se il tuo file non ha intestazioni
        .from_path(path) {
            Ok(r) => r,
            Err(e) => return Err(DbErr::Custom(format!("CSV reader error: {}", e))),
    };

    let mut language_names = Vec::new(); // Pre-allocate for efficiency

    for result in reader.records() {
        match result {
            Ok(record) => {
                let col0 = record.get(0);
                let col3 = record.get(3);

                match (col0, col3) {
                    (Some(val0), Some(val3)) => {
                        language_names.push((val0.to_string(), val3.to_string()));
                    }
                    _ => {
                        return Err(DbErr::Custom("Dati mancanti nel record (colonna 0 o 3)".to_string()));
                    }
                }
            }
            Err(e) => return Err(DbErr::Custom(format!("Errore leggendo un record CSV: {}", e))),
        }
    }

    Ok(language_names)
}

async fn seed_languages_names_table(pool: &SqlitePool) -> Result<(), RitmoErr> {

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone().clone());

    let languages = get_languages_names().await?;

    if languages.is_empty() {
        return Ok(()); // Nothing to insert
    }

    let mut iso_table : Vec<crate::db::entity::languages_names::ActiveModel> = Vec::new();

    for (iso_code, language_name) in languages {
        let iso_single: crate::db::entity::languages_names::ActiveModel = crate::db::entity::languages_names::ActiveModel {
            ref_name: Set(Some(language_name.to_owned())),
            id: Set(iso_code.to_owned()),
            ..Default::default()
        };
        iso_table.push(iso_single);
    }

    let _ = LanguagesNames::insert_many(iso_table).exec(&conn).await?;

    Ok(())
}

pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
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
    
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());    

    // Run migrations if create is true
    if create {
        Migrator::up(&conn, None)
            .await
            .map_err(|e| RitmoErr::DatabaseMigrationFailed(
                format!("Failed to run migrations: {}", e)
            ))?;

        // after migration try to copy all the iso codes and names
        seed_languages_names_table(&pool).await?;
    }

    Ok(pool)
}
