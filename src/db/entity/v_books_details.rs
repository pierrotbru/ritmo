#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_books_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    pub book_title: String,
    pub book_subtitle: Option<String>,
    pub book_original_title: Option<String>,
    pub book_description: Option<String>,
    pub book_isbn: Option<String>,
    pub book_pages: Option<i32>,
    pub book_publication_year: Option<i32>,
    pub book_publisher_name: Option<String>,
    pub book_series_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    Books,
    #[sea_orm(
        belongs_to = "super::publishers::Entity",
        from = "Column::BookPublisherName",
        to = "super::publishers::Column::Name"
    )]
    Publishers,
    #[sea_orm(
        belongs_to = "super::series::Entity",
        from = "Column::BookSeriesName",
        to = "super::series::Column::Name"
    )]
    Series,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::publishers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publishers.def()
    }
}

impl Related<super::series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Series.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods
impl Model {
    pub fn new(
        book_id: i64,
        book_title: String,
        book_subtitle: Option<String>,
        book_original_title: Option<String>,
        book_description: Option<String>,
        book_isbn: Option<String>,
        book_pages: Option<i32>,
        book_publication_year: Option<i32>,
        book_publisher_name: Option<String>,
        book_series_name: Option<String>,
    ) -> Self {
        Self {
            book_id,
            book_title,
            book_subtitle,
            book_original_title,
            book_description,
            book_isbn,
            book_pages,
            book_publication_year,
            book_publisher_name,
            book_series_name,
        }
    }
}