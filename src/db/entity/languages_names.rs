#![allow(non_camel_case_types)]
// src/db/entity/languages_names.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "languages_names")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub language_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    pub is_primary: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::source_languages::Entity",
        from = "Column::LanguageId",
        to = "super::source_languages::Column::Id"
    )]
    source_languages,
    #[sea_orm(
        belongs_to = "super::current_languages::Entity",
        from = "Column::LanguageId",
        to = "super::current_languages::Column::Id"
    )]
    current_languages,
    #[sea_orm(
        belongs_to = "super::original_languages::Entity",
        from = "Column::LanguageId",
        to = "super::original_languages::Column::Id"
    )]
    original_languages,
}

impl Related<super::source_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::source_languages.def()
    }
}

impl Related<super::current_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::current_languages.def()
    }
}

impl Related<super::original_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::original_languages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}