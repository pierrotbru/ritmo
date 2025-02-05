use crate::errors::RitmoErr;
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;

pub async fn sync_publishers(
    calibre_conn: &SqlitePool,
    my_conn: &SqlitePool,
    import_errors: &mut HashMap<String, Vec<(i64, String)>>,
) -> Result<(), RitmoErr> {

    println!("inizio editori");
    // 3. Leggi i dati da books_publishers_link nel database Calibre
    let publisher_links = sqlx::query("SELECT book, publisher FROM books_publishers_link")
        .fetch_all(calibre_conn)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch publisher links from Calibre: {}", e)))?;


    let mut tx = my_conn.begin().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to start publisher sync transaction: {}", e)))?;

    // 4. Esegue gli UPDATE all'interno della transazione PRINCIPALE
    for link in publisher_links {
        let book_id: i64 = link.try_get("book")
            .map_err(|e| RitmoErr::ImportError(format!("Failed to get book id: {}", e)))?;
        let publisher_id: i64 = link.try_get("publisher")
            .map_err(|e| RitmoErr::ImportError(format!("Failed to get publisher id: {}", e)))?;

        let update_result = sqlx::query("UPDATE Books SET publisher_id = ? WHERE id = ?")
            .bind(publisher_id)
            .bind(book_id)
            .execute(&mut *tx)
            .await;

        if let Err(e) = update_result {
            import_errors.entry("Books Update".to_string()).or_default().push((book_id, format!("Update publisher_id for book {} failed: {}", book_id, e)));
        }
    }
        tx.commit().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to commit publisher sync transaction: {}", e)))?;

    Ok(())
}