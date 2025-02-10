#![allow(non_camel_case_types)]

use std::path::Path;
use sea_orm_migration::prelude::*;
use async_trait::async_trait;
use csv::ReaderBuilder; // Use ReaderBuilder for more control
use sea_orm::DbErr;



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
            seed_languages_names_table(manager).await?;

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

async fn get_languages_names() -> Result<Vec<(String, String)>, DbErr> {
    let path = Path::new("./resources/iso-639-2.tab");

    // More robust path handling:
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            // Convert the io::Error to DbErr.  You might want a more specific DbErr variant.
            return Err(DbErr::Custom(format!("Error canonicalizing path: {}", e)));
        }
    };

    let mut reader = match ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false) // Se il tuo file non ha intestazioni
        .from_path(path) {
            Ok(r) => r,
            Err(e) => return Err(DbErr::Custom(format!("CSV reader error: {}", e))),
    };

    let mut language_names = Vec::new(); // Pre-allocate for efficiency

    for result in reader.records() {
        match result {
            Ok(record) => {
                let col0 = record.get(0);
                let col3 = record.get(3);

                match (col0, col3) {
                    (Some(val0), Some(val3)) => {
                        language_names.push((val0.to_string(), val3.to_string()));
                    }
                    _ => {
                        return Err(DbErr::Custom("Dati mancanti nel record (colonna 0 o 3)".to_string()));
                    }
                }
            }
            Err(e) => return Err(DbErr::Custom(format!("Errore leggendo un record CSV: {}", e))),
        }
    }

    Ok(language_names)
}

// Questa funziona me è un loop, molto lento

async fn seed_languages_names_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let languages = get_languages_names().await?;

    if languages.is_empty() {
        return Ok(()); // Nothing to insert
    }

    for (iso_code, language_name) in languages {
        let insert = Query::insert() // Costruisci la query *dentro* il ciclo
            .into_table(languages_names::Table)
            .columns([languages_names::Id, languages_names::RefName])
            .values([iso_code.clone().into(), language_name.into()])
            .map_err(|e| DbErr::Custom(format!("Error inserting query: {}", e)))?
            .to_owned(); // to_owned() *qui* è fondamentale

        if let Err(e) = manager.exec_stmt(insert).await { // Esecuzione della query
            return Err(DbErr::Custom(format!("Errore nell'eseguire la query: {}", e)));
        }
    }

    Ok(())
}
