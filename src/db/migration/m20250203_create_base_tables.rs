#![allow(non_camel_case_types)]

use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Language-related tables
        manager
            .create_table(
                Table::create()
                    .table(current_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(current_languages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(current_languages::LangCode).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(original_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(original_languages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(original_languages::LangCode).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(source_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(source_languages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(source_languages::LangCode).string().not_null())
                    .to_owned(),
            )
            .await?;

        // publishers table
        manager
            .create_table(
                Table::create()
                    .table(publishers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(publishers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(publishers::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // formats table
        manager
            .create_table(
                Table::create()
                    .table(formats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(formats::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(formats::FormatName).string().not_null())
                    .to_owned(),
            )
            .await?;

        // series table
        manager
            .create_table(
                Table::create()
                    .table(series::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(series::SeriesId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(series::series).string().not_null())
                    .to_owned(),
            )
            .await?;

        // people table
        manager
            .create_table(
                Table::create()
                    .table(people::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(people::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(people::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // roles table
        manager
            .create_table(
                Table::create()
                    .table(roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(roles::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
            seed_roles_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order - Corrected
        manager.drop_table(Table::drop().table(roles::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(people::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(series::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(formats::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(publishers::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(source_languages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(original_languages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(current_languages::Table).to_owned()).await?;
        Ok(())
    }
}

// Define enums for table and column names
#[derive(Iden)]
enum current_languages {
    Table,
    Id,
    LangCode,
}

#[derive(Iden)]
enum original_languages {
    Table,
    Id,
    LangCode,
}

#[derive(Iden)]
enum source_languages {
    Table,
    Id,
    LangCode,
}

#[derive(Iden)]
enum publishers {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum formats {
    Table,
    Id,
    FormatName,
}

#[derive(Iden)]
enum series {
    Table,
    SeriesId,
    series,
}

#[derive(Iden)]
enum people {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum roles {
    Table,
    Id,
    Name,
}

async fn seed_roles_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let roles_data = vec![
        "author",
        "translator",
        "curator",
    ];

    for role in roles_data {
        let insert = Query::insert()
            .into_table(roles::Table)
            .columns([roles::Name])
            .values_panic([role.into()])
            .to_owned();
        
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}
