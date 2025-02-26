use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query, Row};
use std::time::Instant;

#[derive(sqlx::FromRow, Debug)]
struct Book {
    id: i64,
    publisher_id: Option<i64>,
}

#[derive(sqlx::FromRow, Debug)]
struct Publisher {
    id: i64,
    name: String,
}

pub async fn sync_publishers(calibre_conn: &SqlitePool, my_conn: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();
    let publisher_links = sqlx::query("SELECT book, publisher FROM books_publishers_link")
        .fetch_all(calibre_conn)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch publisher links from Calibre: {}", e)))?;

    let duration = start.elapsed();
    println!("fetch editors: {:?}", duration);

    let start = Instant::now();
    let mut tx = my_conn.begin().await.map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    for link in publisher_links {
        let book_id: i64 = link.get("book");
        let publisher_id: i64 = link.get("publisher");

        query!("UPDATE Books SET publisher_id = ? WHERE id = ?", publisher_id, book_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    tx.commit().await.map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    let duration = start.elapsed();
    println!("update books: {:?}", duration);

    Ok(())
}

pub async fn import_publishers(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM publishers")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table publishers: {}", e)))?;

    for row in calibre_rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");

        query!("INSERT INTO Publishers (id, name) VALUES (?, ?)", id, name)
            .execute(dst)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    let duration = start.elapsed();
    println!("sqlx import publishers: {:?}", duration);
    Ok(())
}
