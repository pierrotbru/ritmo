use sqlx::Row;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;
use crate::errors::RitmoErr;

#[derive(sqlx::FromRow, Debug)]
struct CurrentLanguage {
    id: i64,
    lang_code: String,
}

pub async fn import_current_languages(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, lang_code FROM languages")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table languages: {}", e)))?;

    for row in calibre_rows {
        let id: i64 = row.get("id");
        let lang_code: String = row.get("lang_code");

        query!("INSERT INTO current_languages (id, lang_code) VALUES (?, ?)", id, lang_code)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import current_languages: {:?}", duration);
    Ok(())
}