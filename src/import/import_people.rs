use sqlx::Row;
use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct Person {
    id: i64,
    name: String,
}

pub async fn import_people(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM authors")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table people: {}", e)))?;

    for row in calibre_rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");

        query!("INSERT INTO People (id, name) VALUES (?, ?)", id, name)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import people: {:?}", duration);
    Ok(())
}