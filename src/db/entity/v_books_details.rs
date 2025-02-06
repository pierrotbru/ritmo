#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_books_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    pub book_title: String,
    pub book_original_title: Option<String>,
    pub publication_date: Option<DateTimeWithTimeZone>,
    pub acquisition_date: Option<DateTimeWithTimeZone>,
    pub last_modified_date: Option<DateTimeWithTimeZone>,
    pub book_notes: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<i64>,
    pub series_index: Option<f64>,
    pub book_tags: Option<String>,
    pub book_people: Option<String>,
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
        from = "Column::PublisherName",
        to = "super::publishers::Column::Name"
    )]
    Publishers,
    #[sea_orm(
        belongs_to = "super::formats::Entity",
        from = "Column::FormatName",
        to = "super::formats::Column::Name"
    )]
    Formats,
    #[sea_orm(
        belongs_to = "super::series::Entity",
        from = "Column::SeriesId",
        to = "super::series::Column::Id"
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

impl Related<super::formats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Formats.def()
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
        book_original_title: Option<String>,
        publication_date: Option<DateTimeWithTimeZone>,
        acquisition_date: Option<DateTimeWithTimeZone>,
        last_modified_date: Option<DateTimeWithTimeZone>,
        book_notes: Option<String>,
        publisher_name: Option<String>,
        format_name: Option<String>,
        series_name: Option<String>,
        series_id: Option<i64>,
        series_index: Option<f64>,
        book_tags: Option<String>,
        book_people: Option<String>,
    ) -> Self {
        Self {
            book_id,
            book_title,
            book_original_title,
            publication_date,
            acquisition_date,
            last_modified_date,
            book_notes,
            publisher_name,
            format_name,
            series_name,
            series_id,
            series_index,
            book_tags,
            book_people,
        }
    }
}