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
                    .table(contents_types::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_types::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(contents_types::TypeName).string().not_null())
                    .to_owned(),
            )
            .await?;
            seed_types_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(contents_types::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum contents_types {
    Table,
    Id,
    TypeName,
}

async fn seed_types_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let types_data = vec![
        "Novel",
        "Short novel",
        "Short story",
        "Essay",
        "Treatise",
        "Dissertation",
        "Biography",
        "Autobiography",
        "Memoir",
        "Interview",
        "Play",
        "One-act play",
        "Poetry",
        "Opera"
    ];

    for contents_type in types_data {
        let insert = Query::insert()
            .into_table(contents_types::Table)
            .columns([contents_types::TypeName])
            .values_panic([contents_type.into()])
            .to_owned();
        
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}
