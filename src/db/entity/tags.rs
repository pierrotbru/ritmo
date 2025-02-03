// src/db/entity/tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub tag_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_tags::Entity")]
    books_tags,
    #[sea_orm(has_many = "super::content_tags::Entity")]
    contents_tags,
}

impl ActiveModelBehavior for ActiveModel {}

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

// src/db/entity/book_contents.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    #[sea_orm(primary_key)]
    pub content_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    books,
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    contents,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/books_tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    #[sea_orm(primary_key)]
    pub tag_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    books,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    tags,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub tag_id: i64,
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
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    tags,
}

impl ActiveModelBehavior for ActiveModel {}

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

// src/db/entity/content_orig_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contents_original_languages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub orig_lang_id: i64,
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
        belongs_to = "super::original_languages::Entity",
        from = "Column::OrigLangId",
        to = "super::original_languages::Column::Id"
    )]
    original_languages,
}

impl ActiveModelBehavior for ActiveModel {}

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

// src/db/entity/lang_names.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "languages_names")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub ref_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/original_languages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "original_languages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub lang_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_orig_lang::Entity")]
    contents_original_languages,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/source_languages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "source_languages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub lang_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_sourc_lang::Entity")]
    contents_source_languages,
}

impl ActiveModelBehavior for ActiveModel {}

