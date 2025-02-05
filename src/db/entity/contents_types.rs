#![allow(non_camel_case_types)]
// src/db/entity/contents_types.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_types")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::contents::Entity")]
    contents,
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}