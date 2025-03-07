use sqlx::Row;
use crate::RitmoErr;
use sqlx::query;
use sqlx::SqlitePool;

#[derive(Default)]
pub struct BookSearchCriteria {
    pub book_name: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series_name: Option<String>,
    pub series_index: Option<i32>,
    pub original_title: Option<String>,
    pub book_notes: Option<String>,
    pub has_cover: Option<bool>,
    pub has_paper: Option<bool>,
    pub file_link: Option<String>,
    pub book_pre_accepted: Option<bool>,
    pub person_name_book: Option<String>,
    pub role_name_book: Option<String>,
    pub tag_name_book: Option<String>,
    pub content_name: Option<String>,
    pub original_title_content: Option<String>,
    pub publication_date_content: Option<i64>,
    pub content_notes: Option<String>,
    pub type_name_content: Option<String>,
    pub content_pre_accepted: Option<bool>,
    pub person_name_content: Option<String>,
    pub role_name_content: Option<String>,
    pub tag_name_content: Option<String>,
    pub language_name: Option<String>,
    pub language_role: Option<String>,
}

pub async fn search_books(pool: &SqlitePool, criteria: &BookSearchCriteria) -> Result<Vec<i32>, RitmoErr> {
    let (query_str, params) = build_query(criteria);
    let mut query = query(&query_str);

    for param in params {
        query = match param {
            Param::String(s) => query.bind(s),
            Param::I64(i) => query.bind(i),
            Param::I32(i) => query.bind(i),
            Param::Bool(b) => query.bind(b),
        };
    }

    let rows = query.fetch_all(pool).await?;

    let ids: Result<Vec<i32>, sqlx::Error> = rows.iter()
        .map(|row| row.try_get(0))
        .collect();

    Ok(ids?) // Usa l'operatore '?' per propagare l'errore
}

pub enum Param {
    String(String),
    I64(i64),
    I32(i32),
    Bool(bool),
}

pub fn build_query(criteria: &BookSearchCriteria) -> (String, Vec<Param>) {
    let mut query = String::from(
        "SELECT bfd.* FROM BooksFullDetails bfd LEFT JOIN books_contents bc ON bfd.book_id = bc.book_id LEFT JOIN ContentsFullDetails cfd ON bc.content_id = cfd.content_id WHERE 1=1",
    );

    let mut params: Vec<Param> = Vec::new();

    if let Some(book_name) = &criteria.book_name {
        query.push_str(" AND bfd.book_name = ?");
        params.push(Param::String(book_name.clone()));
    }

    if let Some(publisher_name) = &criteria.publisher_name {
        query.push_str(" AND bfd.publisher_name = ?");
        params.push(Param::String(publisher_name.clone()));
    }

    if let Some(format_name) = &criteria.format_name {
        query.push_str(" AND bfd.format_name = ?");
        params.push(Param::String(format_name.clone()));
    }

    if let Some(acquisition_date) = &criteria.acquisition_date {
        query.push_str(" AND bfd.acquisition_date = ?");
        params.push(Param::I64(*acquisition_date));
    }

    if let Some(publication_date) = &criteria.publication_date {
        query.push_str(" AND bfd.publication_date = ?");
        params.push(Param::I64(*publication_date));
    }

    if let Some(last_modified_date) = &criteria.last_modified_date {
        query.push_str(" AND bfd.last_modified_date = ?");
        params.push(Param::I64(*last_modified_date));
    }

    if let Some(series_name) = &criteria.series_name {
        query.push_str(" AND bfd.series_name = ?");
        params.push(Param::String(series_name.clone()));
    }

    if let Some(series_index) = &criteria.series_index {
        query.push_str(" AND bfd.series_index = ?");
        params.push(Param::I32(*series_index));
    }

    if let Some(original_title) = &criteria.original_title {
        query.push_str(" AND bfd.original_title = ?");
        params.push(Param::String(original_title.clone()));
    }

    if let Some(book_notes) = &criteria.book_notes {
        query.push_str(" AND bfd.book_notes = ?");
        params.push(Param::String(book_notes.clone()));
    }

    if let Some(has_cover) = &criteria.has_cover {
        query.push_str(" AND bfd.has_cover = ?");
        params.push(Param::Bool(*has_cover));
    }

    if let Some(has_paper) = &criteria.has_paper {
        query.push_str(" AND bfd.has_paper = ?");
        params.push(Param::Bool(*has_paper));
    }

    if let Some(person_name_book) = &criteria.person_name_book {
        query.push_str(" AND bfd.person_name = ?");
        params.push(Param::String(person_name_book.clone()));
    }

    if let Some(role_name_book) = &criteria.role_name_book {
        query.push_str(" AND bfd.role_name = ?");
        params.push(Param::String(role_name_book.clone()));
    }

    if let Some(tag_name_book) = &criteria.tag_name_book {
        query.push_str(" AND bfd.tag_name = ?");
        params.push(Param::String(tag_name_book.clone()));
    }

    if let Some(content_name) = &criteria.content_name {
        query.push_str(" AND cfd.content_name = ?");
        params.push(Param::String(content_name.clone()));
    }

    if let Some(original_title_content) = &criteria.original_title_content {
        query.push_str(" AND cfd.original_title = ?");
        params.push(Param::String(original_title_content.clone()));
    }

    if let Some(publication_date_content) = &criteria.publication_date_content {
        query.push_str(" AND cfd.publication_date = ?");
        params.push(Param::I64(*publication_date_content));
    }

    if let Some(content_notes) = &criteria.content_notes {
        query.push_str(" AND cfd.content_notes = ?");
        params.push(Param::String(content_notes.clone()));
    }

    if let Some(type_name_content) = &criteria.type_name_content {
        query.push_str(" AND cfd.type_name = ?");
        params.push(Param::String(type_name_content.clone()));
    }

    if let Some(person_name_content) = &criteria.person_name_content {
        query.push_str(" AND cfd.person_name = ?");
        params.push(Param::String(person_name_content.clone()));
    }

    if let Some(role_name_content) = &criteria.role_name_content {
        query.push_str(" AND cfd.role_name = ?");
        params.push(Param::String(role_name_content.clone()));
    }

    if let Some(tag_name_content) = &criteria.tag_name_content {
        query.push_str(" AND cfd.tag_name = ?");
        params.push(Param::String(tag_name_content.clone()));
    }

    if let Some(language_name) = &criteria.language_name {
        query.push_str(" AND cfd.language_name = ?");
        params.push(Param::String(language_name.clone()));
    }

    if let Some(language_role) = &criteria.language_role {
        query.push_str(" AND cfd.language_role = ?");
        params.push(Param::String(language_role.clone()));
    }

    (query, params)
}