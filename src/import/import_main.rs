//use sea_orm::DbErr;
//use sea_orm::{DatabaseConnection};
use crate::db::verify_path::verify_path;
use std::path::PathBuf;
use crate::errors::RitmoErr;
use crate::import::import_formats;
use crate::import::import_publishers;
use sqlx::{Row, sqlite::SqliteRow};
use std::collections::HashMap;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions}; 

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
    src : &PathBuf, 
    dst: &PathBuf
) -> Result<HashMap<String, Vec<(i64, String)>>, RitmoErr> {

    let tmpsrc = verify_path(&src, false)?;
    let tmpdst = verify_path(&dst, false)?;

    // create a sqlx connection to the Calibre database
    let options = SqliteConnectOptions::new()
        .filename(tmpsrc)
        .create_if_missing(false);
    let calibre_conn = SqlitePool::connect_with(options).await?;

    // create a sqlx connection to the Calibre database
    let options = SqliteConnectOptions::new()
        .filename(tmpdst)
        .create_if_missing(false);
    let my_conn = SqlitePool::connect_with(options).await?;
    
    let mut import_errors: HashMap<String, Vec<(i64, String)>> = HashMap::new();

    // Define table import configurations
    let table_configs = vec![
        TableImportConfig {
            table_name: "people",
            calibre_query: "SELECT id, name FROM authors",
            my_query: "INSERT OR REPLACE INTO people (id, name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "tags",
            calibre_query: "SELECT id, name FROM tags",
            my_query: "INSERT OR REPLACE INTO tags (id, tag_name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "current_languages",
            calibre_query: "SELECT id, lang_code FROM languages",
            my_query: "INSERT OR REPLACE INTO current_languages (id, lang_code) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "books",
            calibre_query: "SELECT id, title FROM books",
            my_query: "INSERT OR REPLACE INTO books (id, title) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "publishers",
            calibre_query: "SELECT id, name FROM publishers",
            my_query: "INSERT OR REPLACE INTO publishers (id, name) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "books_publisher",
            calibre_query: "SELECT book, publisher FROM books_publishers_link",
            my_query: "INSERT OR REPLACE INTO books_publisher (book_id, publisher_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "contents",
            calibre_query: "SELECT id, title FROM books",
            my_query: "INSERT OR REPLACE INTO contents (id, title) VALUES (?, ?)",
            column_types: &["INTEGER", "TEXT"],
        },
        TableImportConfig {
            table_name: "books_contents",
            calibre_query: "SELECT id FROM books",
            my_query: "INSERT OR REPLACE INTO books_contents (book_id, content_id) VALUES (?, ?)",
            column_types: &["INTEGER"],
        },
        TableImportConfig {
            table_name: "contents_current_languages",
            calibre_query: "SELECT book, lang_code FROM books_languages_link",
            my_query: "INSERT OR REPLACE INTO contents_current_languages (content_id, curr_lang_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "contents_people",
            calibre_query: "SELECT book, author FROM books_authors_link",
            my_query: "INSERT OR REPLACE INTO contents_people (content_id, person_id, role_id) VALUES (?, ?, 1)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "books_tags",
            calibre_query: "SELECT book, tag FROM books_tags_link",
            my_query: "INSERT OR REPLACE INTO books_tags (book_id, tag_id) VALUES (?, ?)",
            column_types: &["INTEGER", "INTEGER"],
        },
        TableImportConfig {
            table_name: "contents_tags",
            calibre_query: "SELECT book, tag FROM books_tags_link",
            my_query: "INSERT OR REPLACE INTO contents_tags (content_id, tag_id) VALUES (?, ?)",
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
            .fetch_all(&calibre_conn)
            .await
            .map_err(|e| RitmoErr::ImportError(
                format!("Failed to fetch rows for table {}: {}", config.table_name, e)
            ))?;

        // Import each row
        for row in calibre_rows {
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


    import_formats::sync_formats(&calibre_conn, &my_conn, &mut import_errors).await?;
    import_publishers::sync_publishers(&calibre_conn, &my_conn, &mut import_errors).await?;

    Ok(import_errors)
}
