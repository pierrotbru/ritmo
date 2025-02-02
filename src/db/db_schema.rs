use crate::errors::RitmoErr;
use sqlx::{Executor, SqlitePool};
use tracing::{info, error};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use regex::Regex;

pub async fn create_db_schema(pool: &SqlitePool) -> Result<(), RitmoErr> {
    // Determine the absolute path to the SQL directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env::current_dir().unwrap().to_string_lossy().into_owned());
    
    let sql_dir = PathBuf::from(manifest_dir).join("src/db/sql");

    info!("Avvio creazione schema database");

    // Move PRAGMA statements outside of the transaction
    match pool.execute("
        PRAGMA journal_mode=WAL;
        PRAGMA synchronous=NORMAL;
        PRAGMA cache_size=10000;
        PRAGMA foreign_keys=ON;
        PRAGMA temp_store=MEMORY;
    ").await {
        Ok(_) => (),
        Err(e) => {
            error!("Errore impostazione pragma: {}", e);
            return Err(RitmoErr::DatabaseCreationFailed(format!("Configurazione pragma fallita: {}", e)));
        }
    }

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Errore inizio transazione: {}", e);
            return Err(RitmoErr::DatabaseCreationFailed(format!("Avvio transazione fallito: {}", e)));
        }
    };

    info!("Pragma impostati");

    match tx.execute("
        CREATE TABLE IF NOT EXISTS laverdure (
            key TEXT PRIMARY KEY,
            value TEXT
        );
    ").await {
        Ok(_) => (),
        Err(e) => {
            error!("Errore creazione tabella laverdure: {}", e);
            return Err(RitmoErr::DatabaseCreationFailed(format!("Creazione tabella metadati fallita: {}", e)));
        }
    }

    info!("Tabella laverdure creata");

    let sql_files = ["create_tables.sql", "create_indexes.sql", "create_views.sql"];

    for sql_file in sql_files {
        let sql_path = sql_dir.join(sql_file);

        let sql = match fs::read_to_string(&sql_path) {
            Ok(content) => content,
            Err(e) => {
                error!("Errore lettura file SQL {}: {} (path: {})", sql_file, e, sql_path.display());
                return Err(RitmoErr::DatabaseCreationFailed(format!(
                    "Errore lettura file SQL {}: {}", sql_file, e
                )));
            }
        };

        match tx.execute(&*sql).await {
            Ok(_) => info!("Eseguito SQL da file: {}", sql_file),
            Err(e) => {
                error!("Errore esecuzione SQL da file {}: {}", sql_file, e);
                return Err(RitmoErr::DatabaseCreationFailed(format!(
                    "Creazione schema fallita per {}: {}", sql_file, e
                )));
            }
        }
    }

    match tx.commit().await { 
        Ok(_) => (),
        Err(e) => {
            error!("Errore commit transazione: {}", e);
            return Err(RitmoErr::DatabaseCreationFailed(format!("Commit transazione fallito: {}", e)));
        }
    }

    info!("Schema database creato");
    Ok(())
}

/// Parses SQL schema and extracts table and column information
pub fn parse_sql_schema(sql_path: &PathBuf) -> Result<(HashSet<String>, HashMap<String, HashSet<String>>, HashSet<String>), std::io::Error> {
    let sql_content = fs::read_to_string(sql_path)?;
    
    let mut tables = HashSet::new();
    let mut columns = HashMap::new();
    let mut views = HashSet::new();

    // Regular expressions to match CREATE TABLE and CREATE VIEW statements
    let table_regex = Regex::new(r"CREATE\s+TABLE\s+IF\s+NOT\s+EXISTS\s+(\w+)\s*\(([^)]+)\)").unwrap();
    let view_regex = Regex::new(r"CREATE\s+VIEW\s+(\w+)\s+AS").unwrap();
    let column_regex = Regex::new(r"(\w+)\s+\w+").unwrap();

    // Find table names and their columns
    for table_capture in table_regex.captures_iter(&sql_content) {
        let table_name = table_capture.get(1).unwrap().as_str().to_string();
        tables.insert(table_name.clone());

        let table_columns: HashSet<String> = column_regex
            .captures_iter(table_capture.get(2).unwrap().as_str())
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .filter(|col| col != "PRIMARY" && col != "FOREIGN" && col != "UNIQUE")
            .collect();

        columns.insert(table_name, table_columns);
    }

    // Find view names
    for view_capture in view_regex.captures_iter(&sql_content) {
        let view_name = view_capture.get(1).unwrap().as_str().to_string();
        views.insert(view_name);
    }

    Ok((tables, columns, views))
}

/// Loads table and column information from SQL schema files
pub async fn load_database_schema() -> Result<(HashSet<String>, HashMap<String, HashSet<String>>, HashSet<String>), std::io::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().into_owned());
    
    let tables_sql_path = PathBuf::from(manifest_dir.clone()).join("src/db/sql/create_tables.sql");
    let views_sql_path = PathBuf::from(manifest_dir).join("src/db/sql/create_views.sql");

    let (tables, columns, mut views) = parse_sql_schema(&tables_sql_path)?;

    // Merge views from the views SQL file
    let (_, _, additional_views) = parse_sql_schema(&views_sql_path)?;
    views.extend(additional_views);

    Ok((tables, columns, views))
}
