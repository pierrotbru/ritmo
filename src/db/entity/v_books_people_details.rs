#![allow(non_camel_case_types)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "v_books_people_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    pub book_title: String,
    #[sea_orm(primary_key)]
    pub person_id: i64,
    pub person_name: String,
    pub role_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    Books,
    #[sea_orm(
        belongs_to = "super::people::Entity",
        from = "Column::PersonId",
        to = "super::people::Column::Id"
    )]
    People,
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleName",
        to = "super::roles::Column::Name"
    )]
    Roles,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::People.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper methods
impl Model {
    pub fn new(
        book_id: i64, 
        book_title: String, 
        person_id: i64, 
        person_name: String, 
        role_name: String
    ) -> Self {
        Self {
            book_id,
            book_title,
            person_id,
            person_name,
            role_name,
        }
    }
}
