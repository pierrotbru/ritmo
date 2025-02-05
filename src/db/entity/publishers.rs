#![allow(non_camel_case_types)]
// src/db/entity/publishers.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "publishers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub country: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books::Entity")]
    books,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::books.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}