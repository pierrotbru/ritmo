use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query, Row};
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct BookTag {
    book_id: i64,
    tag_id: i64,
}

pub async fn import_books_tags(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();
    let mut tx = dst.begin().await?;

    let calibre_rows = sqlx::query("SELECT book, tag FROM books_tags_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table books_tags_link: {}", e)))?;

    for row in calibre_rows {
        let book_id: i64 = row.get("book");
        let tag_id: i64 = row.get("tag");

        query!("INSERT INTO books_tags (book_id, tag_id) VALUES (?, ?)", book_id, tag_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }
    tx.commit().await?;

    let duration = start.elapsed();
    println!("sqlx import books_tags: {:?}", duration);
    Ok(())
}