// src/db/entity/content.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub title: String,
    pub type_id: Option<i64>,
    pub issue_date: Option<i64>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content_types::Entity",
        from = "Column::TypeId",
        to = "super::content_types::Column::Id"
    )]
    contents_types,
}

impl ActiveModelBehavior for ActiveModel {}
