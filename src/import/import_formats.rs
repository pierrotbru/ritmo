use crate::errors::RitmoErr;
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;

// Costante per la query di inserimento (può stare anche qui)
const INSERT_FORMAT_QUERY: &str = "INSERT INTO Formats (format_name) VALUES (?)";

pub async fn sync_formats(
    calibre_conn: &SqlitePool,
    my_conn: &SqlitePool,
    import_errors: &mut HashMap<String, Vec<(i64, String)>>,
) -> Result<(), RitmoErr> {
    // 1. Leggi i formati e gli ID dei libri da Calibre
    let books_formats = sqlx::query("SELECT b.id AS book_id, d.format AS format_name FROM books b INNER JOIN data d ON b.id = d.book")
        .fetch_all(calibre_conn)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch book formats from Calibre: {}", e)))?;

    println!("letti i formati");
    let mut tx = my_conn.begin().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to start format sync transaction: {}", e)))?;

    println!("inizio transazione");
    // 2. Itera sui formati e aggiorna Books
    for book_format in books_formats {
        let book_id: i64 = book_format.try_get("book_id")
            .map_err(|e| RitmoErr::ImportError(format!("Failed to get book id from Calibre data: {}", e)))?;
        let format_name: String = book_format.try_get("format_name")
            .map_err(|e| RitmoErr::ImportError(format!("Failed to get format name from Calibre data: {}", e)))?;

        // 3. Controlla se il formato esiste nel TUO database (e inseriscilo se non esiste)
        let existing_format = sqlx::query("SELECT id FROM Formats WHERE format_name = ?")
            .bind(&format_name)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| RitmoErr::ImportError(format!("Failed to check for existing format: {}", e)))?;

        let format_id: i64 = match existing_format {
            Some(row) => row.try_get("id")
                .map_err(|e| RitmoErr::ImportError(format!("Failed to get existing format id: {}", e)))?,
            None => {
                let insert_result = sqlx::query(INSERT_FORMAT_QUERY)
                    .bind(&format_name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| RitmoErr::ImportError(format!("Failed to insert new format '{}': {}", format_name, e)))?;

                insert_result.last_insert_rowid() as i64
            }
        };

        // 4. Aggiorna la tabella Books con il format_id
        let update_result = sqlx::query("UPDATE Books SET format_id = ? WHERE id = ?")
            .bind(format_id)
            .bind(book_id)
            .execute(&mut *tx)
            .await;

        if let Err(e) = update_result {
            import_errors.entry("Books Update".to_string()).or_default().push((book_id, format!("Update format_id for book {} failed: {}", book_id, e)));
        }
    }
    tx.commit().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to commit format sync transaction: {}", e)))?;

    Ok(())
}