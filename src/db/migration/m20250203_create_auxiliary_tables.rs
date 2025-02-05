#![allow(non_camel_case_types)]

use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // tags table
        manager
            .create_table(
                Table::create()
                    .table(tags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(tags::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(tags::TagName).string().not_null())
                    .to_owned(),
            )
            .await?;

        // aliases table
        manager
            .create_table(
                Table::create()
                    .table(aliases::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(aliases::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(aliases::Alias).string().not_null())
                    .col(ColumnDef::new(aliases::PersonId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_aliases_people")
                            .from(aliases::Table, aliases::PersonId)
                            .to(people::Table, people::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // languages_names table
        manager
            .create_table(
                Table::create()
                    .table(languages_names::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(languages_names::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(languages_names::RefName).string())
                    .to_owned(),
            )
            .await?;
            create_laverdure_table(manager).await?;
            seed_laverdure_table(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager.drop_table(Table::drop().table(languages_names::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(aliases::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(tags::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Laverdure::Table).to_owned()).await?;

        Ok(())
    }
}

// Define enums for table and column names
#[derive(Iden)]
enum tags {
    Table,
    Id,
    TagName,
}

#[derive(Iden)]
enum aliases {
    Table,
    Id,
    Alias,
    PersonId,
}

#[derive(Iden)]
enum languages_names {
    Table,
    Id,
    RefName,
}

// Note: Add references to external enums from other migration files
#[derive(Iden)]
enum people {
    Table,
    Id,
}

#[derive(Iden)]
enum Laverdure {
    Table,
    Key,
    Value,
}

/* this is a future use table, in which I load program and database data */
async fn create_laverdure_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Laverdure::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Laverdure::Key)
                        .text()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Laverdure::Value)
                        .text()
                        .null(),
                )
                .to_owned(),
        )
        .await
}

async fn seed_laverdure_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let laverdure_data = vec![
        ("author", "laverdure"),
        ("program", "ritmo"),
        ("program release", "0.0.0"),
        ("database_version", "0.0.0"),
    ];

    for (key, value) in laverdure_data {
        let insert = Query::insert()
            .into_table(Laverdure::Table)
            .columns([Laverdure::Key, Laverdure::Value])
            .values_panic([key.into(), value.into()])
            .to_owned();
        
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}
