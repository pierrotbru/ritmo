// src/db/entity/book_contents.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    #[sea_orm(primary_key)]
    pub content_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    books,
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    contents,
}

impl ActiveModelBehavior for ActiveModel {}
