#![allow(non_camel_case_types)]

use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // books table
        manager
            .create_table(
                Table::create()
                    .table(books::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(books::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(books::Title).string().not_null())
                    .col(ColumnDef::new(books::PublisherId).integer().null())
                    .col(ColumnDef::new(books::FormatId).integer().null())
                    .col(ColumnDef::new(books::PublicationDate).big_integer().null())
                    .col(ColumnDef::new(books::AcquisitionDate).big_integer().null())
                    .col(ColumnDef::new(books::LastModifiedDate).big_integer().null())
                    .col(ColumnDef::new(books::SeriesId).integer().null())
                    .col(ColumnDef::new(books::SeriesIndex).integer().null())
                    .col(ColumnDef::new(books::OriginalTitle).string().null())
                    .col(ColumnDef::new(books::Notes).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_publisher")
                            .from(books::Table, books::PublisherId)
                            .to(publishers::Table, publishers::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_format")
                            .from(books::Table, books::FormatId)
                            .to(formats::Table, formats::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_series")
                            .from(books::Table, books::SeriesId)
                            .to(series::Table, series::SeriesId)
                    )
                    .to_owned(),
            )
            .await?;

        // books_people junction table
        manager
            .create_table(
                Table::create()
                    .table(books_people::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .col(books_people::BookId)
                            .col(books_people::PersonId)
                            .col(books_people::RoleId)
                    )
                    .col(
                        ColumnDef::new(books_people::BookId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(books_people::PersonId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(books_people::RoleId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_people_book")
                            .from(books_people::Table, books_people::BookId)
                            .to(books::Table, books::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_people_person")
                            .from(books_people::Table, books_people::PersonId)
                            .to(people::Table, people::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_people_role")
                            .from(books_people::Table, books_people::RoleId)
                            .to(roles::Table, roles::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // books_tags junction table
        manager
            .create_table(
                Table::create()
                    .table(books_tags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(books_tags::BookId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(books_tags::TagId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(books_tags::BookId).col(books_tags::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_tags_book")
                            .from(books_tags::Table, books_tags::BookId)
                            .to(books::Table, books::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_tags_tag")
                            .from(books_tags::Table, books_tags::TagId)
                            .to(tags::Table, tags::Id)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager.drop_table(Table::drop().table(books_tags::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(books_people::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(books::Table).to_owned()).await?;
        Ok(())
    }
}

// Define enums for table and column names
#[derive(Iden)]
enum books {
    Table,
    Id,
    Title,
    PublisherId,
    FormatId,
    PublicationDate,
    AcquisitionDate,
    LastModifiedDate,
    SeriesId,
    SeriesIndex,
    OriginalTitle,
    Notes,
}

#[derive(Iden)]
enum books_people {
    Table,
    BookId,
    PersonId,
    RoleId,
}

#[derive(Iden)]
enum books_tags {
    Table,
    BookId,
    TagId,
}

// Note: Add references to external enums from other migration files
#[derive(Iden)]
enum publishers {
    Table,
    Id,
}

#[derive(Iden)]
enum formats {
    Table,
    Id,
}

#[derive(Iden)]
enum series {
    Table,
    SeriesId,
}

#[derive(Iden)]
enum people {
    Table,
    Id,
}

#[derive(Iden)]
enum roles {
    Table,
    Id,
}

#[derive(Iden)]
enum tags {
    Table,
    Id,
}