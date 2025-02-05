#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_contents_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    pub content_title: String,
    pub content_subtitle: Option<String>,
    pub content_original_title: Option<String>,
    pub content_description: Option<String>,
    pub content_type_name: Option<String>,
    pub content_publication_year: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
        content_id: i64,
        content_title: String,
        content_subtitle: Option<String>,
        content_original_title: Option<String>,
        content_description: Option<String>,
        content_type_name: Option<String>,
        content_publication_year: Option<i32>,
    ) -> Self {
        Self {
            content_id,
            content_title,
            content_subtitle,
            content_original_title,
            content_description,
            content_type_name,
            content_publication_year,
        }
    }
}