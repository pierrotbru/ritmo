use sqlx::{sqlite::SqlitePool, query, Row};
use crate::RitmoErr;
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct Book {
    id: i64,
    name: String,
    series_id: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
struct Content {
    id: i64,
    name: String,
}

#[derive(sqlx::FromRow, Debug)]
struct BookContent {
    book_id: i64,
    content_id: i64,
}

pub async fn import_books(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let foreign_keys_enabled: (i32,) = sqlx::query_as("PRAGMA foreign_keys")
        .fetch_one(dst)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    if foreign_keys_enabled.0 != 1 {
        return Err(RitmoErr::DatabaseQueryFailed("Foreign keys are not enabled".to_string()));
    }


    let calibre_rows = sqlx::query("SELECT id, title FROM books")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table books: {}", e)))?;

    for row in &calibre_rows {
        let id: i64 = row.get("id");
        let name: String = row.get("title");

//        query!("INSERT INTO books (id, name, series_id) VALUES (?, ?, ?)", id, name, Option::<i64>::None)
//            .execute(dst)
//            .await
//            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

        query!("INSERT INTO contents (id, name) VALUES (?, ?)", id, name)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

        query!("INSERT INTO books_contents (book_id, content_id) VALUES (?, ?)", id, id)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import books: {:?}", duration);
    Ok(())
}
