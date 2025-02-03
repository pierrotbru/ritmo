// src/db/entity/content_curr_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_current_languages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub curr_lang_id: i64,
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
        belongs_to = "super::current_languages::Entity",
        from = "Column::CurrLangId",
        to = "super::current_languages::Column::Id"
    )]
    current_languages,
}

impl ActiveModelBehavior for ActiveModel {}
