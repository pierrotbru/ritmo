use sqlx::Row;
use crate::errors::RitmoErr;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct ContentPersonRole {
    content_id: i64,
    person_id: i64,
}

pub async fn import_contents_people(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT book, author FROM books_authors_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table books_authors_link: {}", e)))?;

    for row in calibre_rows {
        let content_id: i64 = row.get("book");
        let person_id: i64 = row.get("author");

        query!("INSERT INTO contents_people_roles (content_id, person_id, role_id) VALUES (?, ?, ?)", content_id, person_id, 1)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import contents_people: {:?}", duration);
    Ok(())
}