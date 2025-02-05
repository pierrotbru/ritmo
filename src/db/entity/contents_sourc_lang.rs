#![allow(non_camel_case_types)]
// src/db/entity/contents_sourc_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_sourc_lang")]
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
        belongs_to = "super::source_languages::Entity",
        from = "Column::LanguageId",
        to = "super::source_languages::Column::Id"
    )]
    source_languages,
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents.def()
    }
}

impl Related<super::source_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::source_languages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}