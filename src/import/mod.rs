pub mod import_formats;
pub mod import_publishers;
pub mod import_people;
pub mod import_tags;
pub mod import_current_languages;
pub mod import_books;
pub mod import_books_tags;
pub mod import_contents_people;
pub mod import_contents_current_languages;

use import_contents_people::import_contents_people;
use import_books_tags::import_books_tags;
use import_publishers::import_publishers;
use import_people::import_people;
use import_tags::import_tags;
use import_books::import_books;
use import_current_languages::import_current_languages;
use import_contents_current_languages::import_contents_current_languages;

use crate::errors::RitmoErr;
use sqlx::sqlite::SqlitePool;

/// Import data from Calibre database to the current database
pub async fn copy_data_from_calibre_db(
    calibre_conn : &SqlitePool, 
    my_conn: &SqlitePool
) -> Result<(), RitmoErr> {

    let _ = import_people(calibre_conn, my_conn).await?;
    let _ = import_tags(calibre_conn, my_conn).await?;
    let _ = import_current_languages(calibre_conn, my_conn).await?;
    let _ = import_publishers(calibre_conn, my_conn).await?;
//    let _ = import_books(calibre_conn, my_conn).await?;
//    let _ = import_books_tags(calibre_conn, my_conn).await?;
//    let _ = import_contents_people(calibre_conn, my_conn).await?;
    let _ = import_contents_current_languages(calibre_conn, my_conn).await?;

    let _ = import_formats::sync_formats(&calibre_conn, &my_conn).await?;
    let _ = import_publishers::sync_publishers(&calibre_conn, &my_conn).await?;

    Ok(())
}
