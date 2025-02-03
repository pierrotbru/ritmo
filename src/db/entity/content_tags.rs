// src/db/entity/content_tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub tag_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    contents,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    tags,
}

impl ActiveModelBehavior for ActiveModel {}
