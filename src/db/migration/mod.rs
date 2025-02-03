// src/db/migration/mod.rs
pub mod m20250203_create_base_tables;
pub mod m20250203_create_books_tables;
pub mod m20250203_create_content_tables;
pub mod m20250203_create_books_contents_tables;
pub mod m20250203_create_contents_types_tables;
pub mod m20250203_create_auxiliary_tables;
pub mod m20250203_create_indices_and_views;

use sea_orm_migration::prelude::*;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250203_create_base_tables::Migration),
            Box::new(m20250203_create_books_tables::Migration),
            Box::new(m20250203_create_content_tables::Migration),
            Box::new(m20250203_create_books_contents_tables::Migration),
            Box::new(m20250203_create_contents_types_tables::Migration),
            Box::new(m20250203_create_auxiliary_tables::Migration),
            Box::new(m20250203_create_indices_and_views::Migration),
        ]
    }
}
