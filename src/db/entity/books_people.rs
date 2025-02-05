#![allow(non_camel_case_types)]
// src/db/entity/books_people.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_people")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub book_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub person_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::people::Entity",
        from = "Column::PersonId",
        to = "super::people::Column::Id"
    )]
    people,
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    books,
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id"
    )]
    roles,
}

impl Related<super::people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::people.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::roles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}