#![allow(non_camel_case_types)]
// src/db/entity/contents.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub title: String,
    pub type_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contents_types::Entity",
        from = "Column::TypeId",
        to = "super::contents_types::Column::Id"
    )]
    contents_types,
    #[sea_orm(has_many = "super::contents_orig_lang::Entity")]
    contents_orig_lang,
}

impl Related<super::contents_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents_types.def()
    }
}

impl Related<super::contents_orig_lang::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents_orig_lang.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}