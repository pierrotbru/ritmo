// src/db/entity/content_sourc_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_source_languages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub source_lang_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    contents,
    #[sea_orm(
        belongs_to = "super::source_languages::Entity",
        from = "Column::SourceLangId",
        to = "super::source_languages::Column::Id"
    )]
    source_languages,
}

impl ActiveModelBehavior for ActiveModel {}
