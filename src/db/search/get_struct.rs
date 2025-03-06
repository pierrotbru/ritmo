use sqlx::{ sqlite::SqlitePoolOptions,
            Row,
            migrate,
            migrate::Migrator,
            Pool,
            Sqlite};

use crate::errors::RitmoErr;

static MIGRATOR: Migrator = migrate!();

#[derive(Debug, Default, Clone)]
pub struct TableInfo {
    pub name: String,
    pub column_info: Vec<(String, String)>,
    pub foreign_keys: Vec<(Vec<String>, String, Vec<String>)>,
}

async fn extract_table_names(pool: &Pool<Sqlite>) -> Result<Vec<String>, RitmoErr> {
    let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(pool)
        .await?;

    let table_names = rows
        .iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

    Ok(table_names)
}

async fn extract_column_info(pool: &Pool<Sqlite>, table_name: &str) -> Result<Vec<(String, String)>, RitmoErr> {
    let rows = sqlx::query(&format!("PRAGMA table_info({})", table_name))
        .fetch_all(pool)
        .await?;

    let column_info = rows
        .iter()
        .map(|row| (row.get::<String, _>("name"), row.get::<String, _>("type")))
        .collect();

    Ok(column_info)
}

async fn extract_foreign_keys(pool: &Pool<Sqlite>, table_name: &str) -> Result<Vec<(Vec<String>, String, Vec<String>)>, RitmoErr> {
    let rows = sqlx::query(&format!("PRAGMA foreign_key_list({})", table_name))
        .fetch_all(pool)
        .await?;

    let foreign_keys = rows
        .iter()
        .map(|row| {
            let from_columns = row.get::<String, _>("from").split(',').map(|s| s.trim().to_string()).collect();
            let to_table = row.get::<String, _>("table");
            let to_columns = row.get::<String, _>("to").split(',').map(|s| s.trim().to_string()).collect();
            (from_columns, to_table, to_columns)
        })
        .collect();

    Ok(foreign_keys)
}

async fn create_database_in_memory(create: bool) -> Result<Pool<Sqlite>, RitmoErr> {
    let database_url = "sqlite::memory:"; // Stringa di connessione per SQLite in memoria

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    if create {
        MIGRATOR.run(&pool)
            .await
            .map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
    }

    Ok(pool)
}

pub async fn get_struct_table() -> Result<Vec<TableInfo>, RitmoErr> {
    let pool = create_database_in_memory(true).await.unwrap();

    let mut tables = Vec::<TableInfo>::new();

    let table_names = extract_table_names(&pool).await?;
    for table_name in table_names {
        let mut table = TableInfo::default();
        table.name = table_name.clone();
        table.column_info = extract_column_info(&pool, &table_name).await?;
        table.foreign_keys = extract_foreign_keys(&pool, &table_name).await?;
        tables.push(table);
    }
    drop(pool);

    Ok(tables)
}
