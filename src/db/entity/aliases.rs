// src/db/entity/aliases.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "aliases")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub alias: String,
    pub person_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::people::Entity",
        from = "Column::PersonId",
        to = "super::people::Column::Id"
    )]
    people,
}

impl ActiveModelBehavior for ActiveModel {}
