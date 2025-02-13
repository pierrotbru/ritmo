use sqlx::{Pool, Sqlite, QueryBuilder, Row};
use crate::errors::RitmoErr;

/// Get book IDs for a given person name by searching for contents authored by that person
///
/// # Arguments
///
/// * `pool` - A reference to the SQLite connection pool
/// * `person_name` - The name of the person to search for
///
/// # Returns
///
/// A vector of book IDs associated with contents by the given person
pub async fn get_book_ids_by_person_name(
    pool: &Pool<Sqlite>, 
    person_name: &str
) -> Result<Vec<i64>, RitmoErr> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT DISTINCT b.id AS book_id 
         FROM books b
         JOIN books_contents bc ON b.id = bc.book_id
         JOIN contents c ON bc.content_id = c.id
         JOIN contents_people cp ON c.id = cp.content_id
         JOIN people p ON cp.person_id = p.id
         WHERE p.name LIKE "
    );

    // Add the person name parameter with wildcard search
    query_builder.push_bind(format!("%{}%", person_name));

    // Execute the query and collect book IDs
    let book_ids = query_builder
        .build()
        .fetch_all(pool)
        .await?
        .iter()
        .map(|row| row.try_get::<i64, _>("book_id").unwrap_or(0))
        .filter(|&id| id != 0)
        .collect::<Vec<i64>>();

    Ok(book_ids)
}

/// Get book IDs for a given current language
///
/// # Arguments
///
/// * `pool` - A reference to the SQLite connection pool
/// * `language_name` - The name of the current language to search for
///
/// # Returns
///
/// A vector of book IDs associated with contents in the given current language
pub async fn get_book_ids_by_current_language(
    pool: &Pool<Sqlite>, 
    language_name: &str
) -> Result<Vec<i64>, RitmoErr> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT DISTINCT b.id AS book_id 
         FROM books b
         JOIN books_contents bc ON b.id = bc.book_id
         JOIN contents c ON bc.content_id = c.id
         JOIN contents_current_languages ccl ON c.id = ccl.content_id
         JOIN current_languages cl ON ccl.curr_lang_id = cl.id
         WHERE cl.lang_code IS "
    );

    // Add the language name parameter with wildcard search
    query_builder.push_bind(language_name);

    // Execute the query and collect book IDs
    let book_ids = query_builder
        .build()
        .fetch_all(pool)
        .await?
        .iter()
        .map(|row| row.try_get::<i64, _>("book_id").unwrap_or(0))
        .filter(|&id| id != 0)
        .collect::<Vec<i64>>();

    Ok(book_ids)
}