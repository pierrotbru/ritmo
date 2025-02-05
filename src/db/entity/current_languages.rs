#![allow(non_camel_case_types)]
// src/db/entity/current_languages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "current_languages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::contents_curr_lang::Entity")]
    contents_curr_lang,
    #[sea_orm(has_many = "super::languages_names::Entity")]
    languages_names,
}

impl Related<super::contents_curr_lang::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents_curr_lang.def()
    }
}

impl Related<super::languages_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::languages_names.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}