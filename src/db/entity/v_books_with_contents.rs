#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_books_with_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    pub book_title: String,
    pub book_original_title: Option<String>,
    pub publication_date: Option<Date>,
    pub acquisition_date: Option<Date>,
    pub last_modified_date: Option<Date>,
    pub book_notes: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<i64>,
    pub series_index: Option<i32>,
    pub book_tags: Option<String>,
    pub book_people: Option<String>,
    pub contents_titles: Option<String>,
    pub contents_ids: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    Books,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods
impl Model {
    pub fn get_content_ids(&self) -> Vec<i64> {
        self.contents_ids
            .as_ref()
            .map(|ids| 
                ids.split(',')
                   .filter_map(|id| id.parse().ok())
                   .collect()
            )
            .unwrap_or_default()
    }

    pub fn get_content_titles(&self) -> Vec<String> {
        self.contents_titles
            .as_ref()
            .map(|titles| 
                titles.split(',')
                      .map(|title| title.to_string())
                      .collect()
            )
            .unwrap_or_default()
    }
}