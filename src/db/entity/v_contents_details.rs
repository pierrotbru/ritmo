#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_contents_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    pub content_title: String,
    pub content_original_title: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contents::Entity",
        from = "Column::ContentId",
        to = "super::contents::Column::Id"
    )]
    Contents,
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods
impl Model {
    pub fn new(
        content_id: i64,
        content_title: String,
        content_original_title: Option<String>,
    ) -> Self {
        Self {
            content_id,
            content_title,
            content_original_title,
        }
    }
}