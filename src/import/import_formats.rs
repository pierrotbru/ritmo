use sqlx::{sqlite::SqlitePool, query, Row};
use crate::errors::RitmoErr;

#[derive(sqlx::FromRow, Debug)]
struct Format {
    id: i64,
    name: String,
}

#[derive(sqlx::FromRow, Debug)]
struct Book {
    id: i64,
    format_id: Option<i64>,
}

pub async fn sync_formats(calibre_conn: &SqlitePool, my_conn: &SqlitePool) -> Result<(), RitmoErr> {
    let books_formats = sqlx::query("SELECT b.id AS book_id, d.format AS format_name FROM books b INNER JOIN data d ON b.id = d.book")
        .fetch_all(calibre_conn)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch book formats from Calibre: {}", e)))?;

    let mut tx = my_conn.begin().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to start format sync transaction: {}", e)))?;

    for book_format in books_formats {
        let book_id: i64 = book_format.get("book_id");
        let format_name: String = book_format.get("format_name");

        // Controlla se il formato esiste nel TUO database (e inseriscilo se non esiste)
        let existing_format = sqlx::query("SELECT id, name FROM Formats WHERE name = ?")
            .bind(format_name.clone())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| RitmoErr::ImportError(format!("Failed to check for existing format: {}", e)))?;

        let format_id: i64 = match existing_format {
            Some(row) => {
                let id: i64 = row.get("id");
                id
            }
            None => {
                let insert_result = query!("INSERT INTO Formats (name) VALUES (?)", format_name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| RitmoErr::ImportError(format!("Failed to insert new format '{}': {}", format_name, e)))?;

                insert_result.last_insert_rowid() as i64
            }
        };

        // Aggiorna la tabella Books con il format_id
        query!("UPDATE Books SET format_id = ? WHERE id = ?", format_id, book_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::ImportError(format!("Failed to update Books table: {}", e)))?;
    }

    tx.commit().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to commit format sync transaction: {}", e)))?;

    Ok(())
}
