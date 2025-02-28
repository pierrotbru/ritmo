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
        "INSERT INTO books (name, publisher_id, format_id, publication_date,
        acquisition_date, last_modified_date, series_id, series_index,
        original_title, notes, has_cover, has_paper, file_link, pre_accepted
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        new_book.name, new_book.publisher_id, new_book.format_id, new_book.publication_date,
        new_book.acquisition_date, new_book.last_modified_date, new_book.series_id, new_book.series_index,
        new_book.original_title, new_book.notes, new_book.has_cover, new_book.has_paper, new_book.file_link, new_book.pre_accepted
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert content: {}", e)))?;

    let new_book_id = result.last_insert_rowid();

    // mancano alcune tabelle join
    for tag in &book.tags.clone() {
        let result = search_and_add(&mut tx, "tags", "id", "name", &tag, IdAction::AddId )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add tag {}: {}", tag, e)))?;
        let tag_id = result.id.ok_or(RitmoErr::DatabaseInsertFailed("Tag not found or added".to_string()))?;

        query!(
            "INSERT INTO books_tags (book_id, tag_id) VALUES (?, ?)",
            new_book_id,
            tag_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert book tags: {}", e)))?;
    }

    for (person_name, role_name) in &book.people {
        let person_result = search_and_add(&mut tx, "people", "id", "name", person_name, IdAction::AddId )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add person {}: {}", person_name, e)))?;

        let person_id = person_result.id.ok_or(RitmoErr::DatabaseInsertFailed(format!("Person {} not found or added", person_name )))?;

        let role_result = search_and_add( &mut tx, "roles", "id", "name", role_name, IdAction::AddId )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add role {}: {}", role_name, e)))?;

        let role_id = role_result.id.ok_or(RitmoErr::DatabaseInsertFailed(format!("Role {} not found or added", role_name )))?;

        query!(
            "INSERT INTO books_people_roles (book_id, person_id, role_id) VALUES (?, ?, ?)",
            new_book_id, person_id, role_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            RitmoErr::DatabaseInsertFailed(format!("Failed to insert into books_people_roles: {}", e))
        })?;
    }


    tx.commit()
      .await
      .map_err(|e| RitmoErr::TransactionCommitFailed(e.to_string()))?;

    Ok(new_book_id.try_into().unwrap())
}