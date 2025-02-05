#![allow(non_camel_case_types)]

use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(books_contents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(books_contents::BookId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(books_contents::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(books_contents::BookId).col(books_contents::ContentId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_books_contents_book")
                            .from(books_contents::Table, books_contents::BookId)
                            .to(books::Table, books::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_books_contents_content")
                            .from(books_contents::Table, books_contents::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(books_contents::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum books_contents {
    Table,
    BookId,
    ContentId,
}

// Import external enums from other migration files
#[derive(Iden)]
enum books {
    Table,
    Id,
}

#[derive(Iden)]
enum contents {
    Table,
    Id,
}