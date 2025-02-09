#![allow(non_camel_case_types)]
// src/db/entity/series.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "series")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
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