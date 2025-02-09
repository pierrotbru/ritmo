#![allow(non_camel_case_types)]
// src/db/entity/roles.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_people::Entity")]
    books_people,
    #[sea_orm(has_many = "super::contents_people::Entity")]
    contents_people,
}

impl Related<super::books_people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::books_people.def()
    }
}

impl Related<super::contents_people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::contents_people.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}