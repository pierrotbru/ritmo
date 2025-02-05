#![allow(non_camel_case_types)]
// src/db/entity/books.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher_id: Option<i64>,
    pub series_id: Option<i64>,
    pub format_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub pages: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::publishers::Entity", 
        from = "Column::PublisherId", 
        to = "super::publishers::Column::Id"
    )]
    publishers,
    
    #[sea_orm(
        belongs_to = "super::series::Entity", 
        from = "Column::SeriesId", 
        to = "super::series::Column::Id"
    )]
    series,
    
    #[sea_orm(
        belongs_to = "super::formats::Entity", 
        from = "Column::FormatId", 
        to = "super::formats::Column::Id"
    )]
    formats,
}

impl Related<super::formats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::formats.def()
    }
}

impl Related<super::publishers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::publishers.def()
    }
}

impl Related<super::series::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::series.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}