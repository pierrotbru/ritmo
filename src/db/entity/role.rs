// src/db/entity/role.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub role_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::book_people::Entity")]
    books_people,
    #[sea_orm(has_many = "super::content_people::Entity")]
    contents_people,
}

impl ActiveModelBehavior for ActiveModel {}
