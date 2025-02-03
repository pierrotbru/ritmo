// src/db/entity/books.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub title: String,
    pub publisher_id: Option<i64>,
    pub format_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series_id: Option<i64>,
    pub series_index: Option<i32>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::publishers::Entity",
        from = "Column::PublisherId",
        to = "super::publishers::Column::Id"
    )]
    Publishers,
    
    #[sea_orm(
        belongs_to = "super::formats::Entity",
        from = "Column::FormatId",
        to = "super::formats::Column::Id"
    )]
    Formats,
    
    #[sea_orm(
        belongs_to = "super::series::Entity",
        from = "Column::SeriesId",
        to = "super::series::Column::SeriesId"
    )]
    Series,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/people.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "People")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub name: String,
    pub nationality: Option<String>,
    pub birth_year: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::aliases::Entity")]
    Aliases,
    #[sea_orm(has_many = "super::book_people::Entity")]
    BookPeople,
    #[sea_orm(has_many = "super::content_people::Entity")]
    ContentPeople,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Content")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub title: String,
    pub type_id: Option<i64>,
    pub issue_date: Option<i64>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content_types::Entity",
        from = "Column::TypeId",
        to = "super::content_types::Column::Id"
    )]
    ContentTypes,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/book_people.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "BookPeople")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub book_id: i64,
    #[sea_orm(primary_key)]
    pub person_id: i64,
    #[sea_orm(primary_key)]
    pub role_id: i64,
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
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id"
    )]
    Role,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_people.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentPeople")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub content_id: i64,
    #[sea_orm(primary_key)]
    pub person_id: i64,
    #[sea_orm(primary_key)]
    pub role_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    Content,
    #[sea_orm(
        belongs_to = "super::people::Entity",
        from = "Column::PersonId",
        to = "super::people::Column::Id"
    )]
    People,
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id"
    )]
    Role,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/current_languages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "CurrentLanguages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub lang_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_types.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentTypes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub type_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content::Entity")]
    Content,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/formats.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Formats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub format_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books::Entity")]
    Books,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/publishers.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Publishers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books::Entity")]
    Books,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/role.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub role_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::book_people::Entity")]
    BookPeople,
    #[sea_orm(has_many = "super::content_people::Entity")]
    ContentPeople,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/series.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Series")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub series_id: i64,
    pub series: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books::Entity")]
    Books,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub tag_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_tags::Entity")]
    BooksTags,
    #[sea_orm(has_many = "super::content_tags::Entity")]
    ContentTags,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/aliases.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Aliases")]
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
    People,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/book_contents.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "BookContents")]
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
    Books,
    #[sea_orm(
        belongs_to = "super::content::Entity",
        from = "Column::ContentId",
        to = "super::content::Column::Id"
    )]
    Content,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/books_tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "BooksTags")]
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
    Books,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    Tags,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_tags.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentTags")]
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
    Content,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    Tags,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_curr_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentCurrLang")]
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
    Content,
    #[sea_orm(
        belongs_to = "super::current_languages::Entity",
        from = "Column::CurrLangId",
        to = "super::current_languages::Column::Id"
    )]
    CurrentLanguages,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_orig_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentOrigLang")]
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
    Content,
    #[sea_orm(
        belongs_to = "super::original_languages::Entity",
        from = "Column::OrigLangId",
        to = "super::original_languages::Column::Id"
    )]
    OriginalLanguages,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/content_sourc_lang.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ContentSourcLang")]
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
    Content,
    #[sea_orm(
        belongs_to = "super::source_languages::Entity",
        from = "Column::SourceLangId",
        to = "super::source_languages::Column::Id"
    )]
    SourceLanguages,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/lang_names.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "LangNames")]
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
#[sea_orm(table_name = "OriginalLanguages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub lang_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_orig_lang::Entity")]
    ContentOrigLang,
}

impl ActiveModelBehavior for ActiveModel {}

// src/db/entity/source_languages.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "SourceLanguages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i64,
    pub lang_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::content_sourc_lang::Entity")]
    ContentSourcLang,
}

impl ActiveModelBehavior for ActiveModel {}

