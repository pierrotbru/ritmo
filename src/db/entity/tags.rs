#![allow(non_camel_case_types)]
// src/db/entity/tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_tags::Entity")]
    books_tags,
    #[sea_orm(has_many = "super::contents_tags::Entity")]
    contents_tags,
}

impl Related<super::books_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::books_tags.def()
    }
}

impl Related<super::contents_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents_tags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}