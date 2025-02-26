use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query, Row};
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct DestTag {
    id: i64,
    name: String,
}

pub async fn import_tags(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = query("SELECT id, name FROM tags")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table tags: {}", e)))?;

    for row in calibre_rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");

        query!("INSERT INTO tags (id, name) VALUES (?, ?)", id, name)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import tags: {:?}", duration);
    Ok(())
}
