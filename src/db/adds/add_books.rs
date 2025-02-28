use sqlx::query;
use crate::db::adds::search_and_add::*;
use crate::RitmoErr;
use sqlx::SqlitePool;

#[derive(Debug, Default)]
pub struct BookData {
    pub name: String,
    pub publisher: Option<String>,
    pub format: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series: Option<String>,
    pub series_index: Option<i32>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
    pub has_cover: Option<i32>,
    pub has_paper: Option<i32>,
    pub file_link: Option<String>,
    pub tags: Vec<String>,
    pub people: Vec<(String, String)>,
    pub pre_accepted: Option<i32>,
}

#[derive(Debug, Default)]
pub struct NewBook {
    pub id: i32,
    pub name: String,
    pub publisher_id: Option<i32>,
    pub format_id: Option<i32>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series_id: Option<i32>,
    pub series_index: Option<i32>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
    pub has_cover: Option<i32>,
    pub has_paper: Option<i32>,
    pub file_link: Option<String>,
    pub pre_accepted: Option<i32>,
}

#[derive(sqlx::FromRow)]
struct IdResult {
    id: i32,
}

pub async fn add_book(pool: SqlitePool, book: &BookData) -> Result<i32, RitmoErr> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    let mut new_book = NewBook {
    name : book.name.clone(),
    publication_date : book.publication_date.clone(),
    acquisition_date : book.acquisition_date.clone(),
    last_modified_date : book.last_modified_date.clone(),
    series_index : book.series_index.clone(),
    original_title : book.original_title.clone(),
    notes : book.notes.clone(),
    has_cover : book.has_cover.clone(),
    has_paper : book.has_paper.clone(),
    file_link : book.file_link.clone(),
    pre_accepted : book.pre_accepted.clone(),
    ..Default::default()
    };

    if let Some(publisher) = &book.publisher {
        let result = search_and_add(
            &mut tx,
            "publishers",
            "id",
            "name",
            publisher,
            IdAction::AddId,
        )
        .await
        .map_err(|e| {
            RitmoErr::SearchAndAddFailed(format!("Failed to search and add publisher {}: {}", publisher, e))
        })?;
        new_book.publisher_id = result.id;
    }

    if let Some(format) = &book.format {
        let result = search_and_add(
            &mut tx,
            "formats",
            "id",
            "name",
            format,
            IdAction::AddId,
        )
        .await
        .map_err(|e| {
            RitmoErr::SearchAndAddFailed(format!("Failed to search and add format {}: {}", format, e))
        })?;
        new_book.format_id = result.id;
    }

    if let Some(series) = &book.series {
        let result = search_and_add(
            &mut tx,
            "series",
            "id",
            "name",
            series,
            IdAction::AddId,
        )
        .await
        .map_err(|e| {
            RitmoErr::SearchAndAddFailed(format!("Failed to search and add series {}: {}", series, e))
        })?;
        new_book.series_id = result.id;
    }

    let result = query!(
        "INSERT INTO books (name) VALUES (?)",
        new_book.name,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert content: {}", e)))?;

    // get the id of the last added record in the books table
    let new_book_id = result.last_insert_rowid();

    // commit the transaction
    tx.commit()
      .await
      .map_err(|e| RitmoErr::TransactionCommitFailed(e.to_string()))?;

    Ok(new_book_id.try_into().unwrap())
}