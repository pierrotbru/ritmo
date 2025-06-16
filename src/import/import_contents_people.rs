use sqlx::Row;
use crate::errors::RitmoErr;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;

pub async fn import_contents_people(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
    let start = Instant::now();
    let mut tx = dst.begin().await?;

    let calibre_rows = sqlx::query("SELECT book, author FROM books_authors_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table books_authors_link: {}", e)))?;

    for row in calibre_rows {
        let content_id: i64 = row.get("book");
        let person_id: i64 = row.get("author");

        // Controlla che il content_id esista in contents
        let content_exists: (i64,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM contents WHERE id = ?)")
            .bind(content_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| RitmoErr::ImportError(format!("Failed to check content_id: {}", e)))?;

        // Controlla che il person_id esista in people
        let person_exists: (i64,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM people WHERE id = ?)")
            .bind(person_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| RitmoErr::ImportError(format!("Failed to check person_id: {}", e)))?;

        if content_exists.0 == 1 && person_exists.0 == 1 {
            query!("INSERT INTO contents_people_roles (content_id, person_id, role_id) VALUES (?, ?, ?)", content_id, person_id, 1)
                .execute(&mut *tx)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
        } else {
            // Puoi loggare o gestire i casi in cui non esistono
            // e.g., dbg!(content_id, person_id, "Skipped: id non trovato");
        }
    }
    tx.commit().await?;

    let duration = start.elapsed();
    dbg!(duration);
    Ok(())
}
