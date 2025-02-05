#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// Define an enum for the different view types
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum ViewType {
    #[sea_orm(string_value = "BooksPeopleDetails")]
    #[serde(rename = "books_people_details")]
    BooksPeopleDetails,
    #[sea_orm(string_value = "ContentsPeopleDetails")]
    #[serde(rename = "contents_people_details")]
    ContentsPeopleDetails,
    #[sea_orm(string_value = "BooksDetails")]
    #[serde(rename = "books_details")]
    BooksDetails,
    #[sea_orm(string_value = "ContentsDetails")]
    #[serde(rename = "contents_details")]
    ContentsDetails,
    #[sea_orm(string_value = "BooksWithContents")]
    #[serde(rename = "books_with_contents")]
    BooksWithContents,
}

// Define the Views entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "views")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub view_name: String,
    pub view_type: ViewType,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooksPeopleDetails {
    pub book_id: i64,
    pub book_title: String,
    pub person_id: i64,
    pub person_name: String,
    pub role_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentsPeopleDetails {
    pub content_id: i64,
    pub content_title: String,
    pub person_id: i64,
    pub person_name: String,
    pub role_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooksDetails {
    pub book_id: i64,
    pub book_title: String,
    pub book_original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub book_notes: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
    pub book_tags: Option<String>,
    pub book_people: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentsDetails {
    pub content_id: i64,
    pub content_title: String,
    pub content_original_title: Option<String>,
    pub issue_date: Option<i64>,
    pub content_notes: Option<String>,
    pub type_name: Option<String>,
    pub content_tags: Option<String>,
    pub content_people: Option<String>,
    pub content_current_languages: Option<String>,
    pub content_original_languages: Option<String>,
    pub content_source_languages: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooksWithContents {
    pub book_id: i64,
    pub book_title: String,
    pub book_original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub book_notes: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
    pub book_tags: Option<String>,
    pub book_people: Option<String>,
    pub contents_titles: Option<String>,
    pub contents_ids: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Define any relationships if needed
}

impl ActiveModelBehavior for ActiveModel {}
