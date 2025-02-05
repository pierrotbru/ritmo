#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_books_with_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    pub book_title: String,
    #[sea_orm(primary_key)]
    pub content_id: i64,
    pub content_title: String,
    pub content_type_name: Option<String>,
    pub content_publication_year: Option<i32>,
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
        belongs_to = "super::contents::Entity",
        from = "Column::ContentId",
        to = "super::contents::Column::Id"
    )]
    Contents,
    #[sea_orm(
        belongs_to = "super::contents_types::Entity",
        from = "Column::ContentTypeName",
        to = "super::contents_types::Column::TypeName"
    )]
    ContentsTypes,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contents.def()
    }
}

impl Related<super::contents_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentsTypes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods
impl Model {
    pub fn new(
        book_id: i64,
        book_title: String,
        content_id: i64,
        content_title: String,
        content_type_name: Option<String>,
        content_publication_year: Option<i32>,
    ) -> Self {
        Self {
            book_id,
            book_title,
            content_id,
            content_title,
            content_type_name,
            content_publication_year,
        }
    }
}