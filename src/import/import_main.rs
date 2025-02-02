use crate::errors::RitmoErr;
use crate::import::import_formats;
use crate::import::import_publishers;
use sqlx::{SqlitePool, Row, sqlite::SqliteRow};
use std::collections::HashMap;

/// Represents a table import configuration
struct TableImportConfig {
    table_name: &'static str,
    calibre_query: &'static str,
    my_query: &'static str,
    column_types: &'static [&'static str],
}

/// Safely convert a row value to the desired type
fn get_row_value<T: sqlx::Type<sqlx::Sqlite> + for<'r> sqlx::Decode<'r, sqlx::Sqlite>>(
    row: &SqliteRow, 
    index: usize
) -> Result<T, RitmoErr> {
    row.try_get(index)
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to parse column at index {}: {}", index, e)
        ))
}

/// Import data from Calibre database to the current database
pub async fn copy_data_from_calibre_db(
    calibre_conn: &SqlitePool, 
    my_conn: &SqlitePool
) -> Result<HashMap<String, Vec<(i64, String)>>, RitmoErr> {
    let mut import_errors: HashMap<String, Vec<(i64, String)>> = HashMap::new();

    // Define table import configurations
    let table_configs = vec![
        TableImportConfig {
            table_name: "People",
            calibre_query: "SELECT id, name FROM authors",
            my_query: "INSERT OR REPLACE INTO People (id, name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "Tags",
            calibre_query: "SELECT id, name FROM tags",
            my_query: "INSERT OR REPLACE INTO Tags (id, tag_name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "CurrentLanguages",
            calibre_query: "SELECT id, lang_code FROM languages",
            my_query: "INSERT OR REPLACE INTO CurrentLanguages (id, lang_code) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "Books",
            calibre_query: "SELECT id, title FROM books",
            my_query: "INSERT OR REPLACE INTO Books (id, title) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "Publishers",
            calibre_query: "SELECT id, name FROM publishers",
            my_query: "INSERT OR REPLACE INTO Publishers (id, name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "BookPublisher",
            calibre_query: "SELECT book, publisher FROM books_publishers_link",
            my_query: "INSERT OR REPLACE INTO BookPublisher (book_id, publisher_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "Content",
            calibre_query: "SELECT id, title FROM books",
            my_query: "INSERT OR REPLACE INTO Content (id, title) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "BookContents",
            calibre_query: "SELECT id FROM books",
            my_query: "INSERT OR REPLACE INTO BookContents (book_id, content_id) VALUES (?, ?)",
            column_types: &["INTEGER"],
        },
        TableImportConfig {
            table_name: "ContentCurrLang",
            calibre_query: "SELECT book, lang_code FROM books_languages_link",
            my_query: "INSERT OR REPLACE INTO ContentCurrLang (content_id, curr_lang_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "ContentPeople",
            calibre_query: "SELECT book, author FROM books_authors_link",
            my_query: "INSERT OR REPLACE INTO ContentPeople (content_id, person_id, role_id) VALUES (?, ?, 1)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "BooksTags",
            calibre_query: "SELECT book, tag FROM books_tags_link",
            my_query: "INSERT OR REPLACE INTO BooksTags (book_id, tag_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "ContentTags",
            calibre_query: "SELECT book, tag FROM books_tags_link",
            my_query: "INSERT OR REPLACE INTO ContentTags (content_id, tag_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
    ];

    // Start a transaction
    let mut tx = my_conn.begin().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to start transaction: {}", e)))?;

    // Iterate through table configurations and import data

    println!("inizio loop");
    for config in &table_configs {
        // Fetch rows from Calibre database
        let calibre_rows = sqlx::query(config.calibre_query)
            .fetch_all(calibre_conn)
            .await
            .map_err(|e| RitmoErr::ImportError(
                format!("Failed to fetch rows for table {}: {}", config.table_name, e)
            ))?;

        // Import each row
        for row in calibre_rows {
            // Dynamic binding based on column types
            match config.column_types.len() {
                1 => {
                    let id: i64 = get_row_value(&row, 0)?;
                    
                    let result = sqlx::query(config.my_query)
                        .bind(id)
                        .bind(id)
                        .execute(&mut *tx)
                        .await;

                    if let Err(e) = result {
                        import_errors.entry(config.table_name.to_string())
                            .or_default()
                            .push((id, format!("Failed to import: {}", e)));
                    }
                },
                2 => {
                    // Dynamically handle INTEGER or TEXT for second column
                    let id: i64 = get_row_value(&row, 0)?;
                    
                    let result = if config.column_types[1] == "INTEGER" {
                        let second_val: i64 = get_row_value(&row, 1)?;
                        sqlx::query(config.my_query)
                            .bind(id)
                            .bind(second_val)
                            .execute(&mut *tx)
                            .await
                    } else {
                        let second_val: String = get_row_value(&row, 1)?;
                        sqlx::query(config.my_query)
                            .bind(id)
                            .bind(second_val)
                            .execute(&mut *tx)
                            .await
                    };

                    if let Err(e) = result {
                        import_errors.entry(config.table_name.to_string())
                            .or_default()
                            .push((id, format!("Failed to import: {}", e)));
                    }
                },
                _ => return Err(RitmoErr::ImportError(
                    format!("Unsupported column count for table {}", config.table_name)
                )),
            }
        }
    }

    // Commit transaction
    tx.commit().await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to commit transaction: {}", e)))?;


    import_formats::sync_formats(calibre_conn, my_conn, &mut import_errors).await?;
    import_publishers::sync_publishers(calibre_conn, my_conn, &mut import_errors).await?;

    // Return import errors if any
    Ok(import_errors)
}
