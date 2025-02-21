use crate::import::import_contents_people::import_contents_people;
use crate::import::import_books_tags::import_books_tags;
use crate::import::import_publishers::import_publishers;
use crate::import::import_people::import_people;
use crate::import::import_tags::import_tags;
use crate::import::import_books::import_books;
use crate::import::import_current_languages::import_current_languages;
use crate::import::import_contents_current_languages::import_contents_current_languages;

use crate::errors::RitmoErr;
use crate::import::*;

use sqlx::{
    Row,
    sqlite::SqliteRow,
    sqlite::SqlitePool,
};
use std::collections::HashMap;

/// Represents a table import configuration
//struct TableImportConfig {
//    table_name: &'static str,
//    calibre_query: &'static str,
//    my_query: &'static str,
//    column_types: &'static [&'static str],
//}
//
/// Safely convert a row value to the desired type
//fn get_row_value<T: sqlx::Type<sqlx::Sqlite> + for<'r> sqlx::Decode<'r, sqlx::Sqlite>>(
//    row: &SqliteRow, 
//    index: usize
//) -> Result<T, RitmoErr> {
//    row.try_get(index)
//        .map_err(|e| RitmoErr::ImportError(
//            format!("Failed to parse column at index {}: {}", index, e)
//        ))
//}

/// Import data from Calibre database to the current database
pub async fn copy_data_from_calibre_db(
    calibre_conn : &SqlitePool, 
    my_conn: &SqlitePool
) -> Result<(), RitmoErr> {

    let _ = import_people(calibre_conn, my_conn).await?;
    let _ = import_tags(calibre_conn, my_conn).await?;
    let _ = import_current_languages(calibre_conn, my_conn).await?;
    let _ = import_books(calibre_conn, my_conn).await?;
    let _ = import_publishers(calibre_conn, my_conn).await?;
    let _ = import_books_tags(calibre_conn, my_conn).await?;
    let _ = import_contents_people(calibre_conn, my_conn).await?;
    let _ = import_contents_current_languages(calibre_conn, my_conn).await?;

    let _ = import_formats::sync_formats(&calibre_conn, &my_conn).await?;
    let _ = import_publishers::sync_publishers(&calibre_conn, &my_conn).await?;

    Ok(())
}
