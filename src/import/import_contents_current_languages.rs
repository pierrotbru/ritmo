use sqlx::Row;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;
use crate::RitmoErr;

#[derive(sqlx::FromRow, Debug)]
struct ContentCurrentLanguage {
    content_id: i64,
    curr_lang_id: String,
}

pub async fn import_contents_current_languages(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT book, lang_code FROM books_languages_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table books_languages_link: {}", e)))?;

    for row in calibre_rows {
        let content_id: i64 = row.get("book");
        let curr_lang_id: String = row.get("lang_code");

        query!("INSERT INTO contents_current_languages (content_id, curr_lang_id) VALUES (?, ?)", content_id, curr_lang_id)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import contents_current_languages: {:?}", duration);
    Ok(())
}
