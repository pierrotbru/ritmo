use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // contents table
        manager
            .create_table(
                Table::create()
                    .table(contents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(contents::Title).string().not_null())
                    .col(ColumnDef::new(contents::OriginalTitle).string().null())
                    .col(ColumnDef::new(contents::PublicationDate).big_integer().null())
                    .col(ColumnDef::new(contents::Notes).string().null())
                    .col(ColumnDef::new(contents::TypeId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contents_type")
                            .from(contents::Table, contents::TypeId)
                            .to(contents_types::Table, contents_types::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // contents_people junction table
        manager
            .create_table(
                Table::create()
                    .table(contents_people::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_people::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_people::PersonId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_people::RoleId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(contents_people::ContentId)
                            .col(contents_people::PersonId)
                            .col(contents_people::RoleId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_people_content")
                            .from(contents_people::Table, contents_people::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_people_person")
                            .from(contents_people::Table, contents_people::PersonId)
                            .to(people::Table, people::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_people_role")
                            .from(contents_people::Table, contents_people::RoleId)
                            .to(roles::Table, roles::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // contents_tags junction table
        manager
            .create_table(
                Table::create()
                    .table(contents_tags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_tags::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_tags::TagId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(contents_tags::ContentId).col(contents_tags::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_tags_content")
                            .from(contents_tags::Table, contents_tags::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_tags_tag")
                            .from(contents_tags::Table, contents_tags::TagId)
                            .to(tags::Table, tags::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // contents Current Language table
        manager
            .create_table(
                Table::create()
                    .table(contents_current_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_current_languages::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_current_languages::CurrLangId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(contents_current_languages::ContentId).col(contents_current_languages::CurrLangId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_curr_lang_content")
                            .from(contents_current_languages::Table, contents_current_languages::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_curr_lang_language")
                            .from(contents_current_languages::Table, contents_current_languages::CurrLangId)
                            .to(current_languages::Table, current_languages::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // contents Original Language table
        manager
            .create_table(
                Table::create()
                    .table(contents_original_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_original_languages::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_original_languages::OrigLangId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(contents_original_languages::ContentId).col(contents_original_languages::OrigLangId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_orig_lang_content")
                            .from(contents_original_languages::Table, contents_original_languages::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_orig_lang_language")
                            .from(contents_original_languages::Table, contents_original_languages::OrigLangId)
                            .to(original_languages::Table, original_languages::Id)
                    )
                    .to_owned(),
            )
            .await?;

        // contents Source Language table
        manager
            .create_table(
                Table::create()
                    .table(contents_source_languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(contents_source_languages::ContentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(contents_source_languages::SourceLangId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(Index::create().col(contents_source_languages::ContentId).col(contents_source_languages::SourceLangId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_sourc_lang_content")
                            .from(contents_source_languages::Table, contents_source_languages::ContentId)
                            .to(contents::Table, contents::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_content_sourc_lang_language")
                            .from(contents_source_languages::Table, contents_source_languages::SourceLangId)
                            .to(source_languages::Table, source_languages::Id)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager.drop_table(Table::drop().table(contents_source_languages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(contents_original_languages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(contents_current_languages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(contents_tags::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(contents_people::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(contents::Table).to_owned()).await?;
        Ok(())
    }
}

// Define enums for table and column names
#[derive(Iden)]
enum contents {
    Table,
    Id,
    Title,
    OriginalTitle,
    PublicationDate,
    Notes,
    TypeId,
}

#[derive(Iden)]
enum contents_people {
    Table,
    ContentId,
    PersonId,
    RoleId,
}

#[derive(Iden)]
enum contents_tags {
    Table,
    ContentId,
    TagId,
}

#[derive(Iden)]
enum contents_current_languages {
    Table,
    ContentId,
    CurrLangId,
}

#[derive(Iden)]
enum contents_original_languages {
    Table,
    ContentId,
    OrigLangId,
}

#[derive(Iden)]
enum contents_source_languages {
    Table,
    ContentId,
    SourceLangId,
}

// Note: Add references to external enums from other migration files
#[derive(Iden)]
enum current_languages {
    Table,
    Id,
}

#[derive(Iden)]
enum original_languages {
    Table,
    Id,
}

#[derive(Iden)]
enum source_languages {
    Table,
    Id,
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

#[derive(Iden)]
enum contents_types {
    Table,
    Id,
}