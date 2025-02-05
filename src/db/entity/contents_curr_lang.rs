#![allow(non_camel_case_types)]
// src/db/entity/contents_curr_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_curr_lang")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub content_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub language_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contents::Entity",
        from = "Column::ContentId",
        to = "super::contents::Column::Id"
    )]
    contents,
    #[sea_orm(
        belongs_to = "super::current_languages::Entity",
        from = "Column::LanguageId",
        to = "super::current_languages::Column::Id"
    )]
    current_languages,
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents.def()
    }
}

impl Related<super::current_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::current_languages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}