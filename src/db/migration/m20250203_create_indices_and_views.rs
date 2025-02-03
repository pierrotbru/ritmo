use sea_orm_migration::prelude::*;
use async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        
        // Include SQL statements from external files
        let create_indices_sql = include_str!("../sql/create_indices.sql");
        let create_views_sql = include_str!("../sql/create_views.sql");
        
        // Execute indices creation
        db.execute_unprepared(create_indices_sql).await?;
        
        // Execute views creation
        db.execute_unprepared(create_views_sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        
        let drop_indices_sql = include_str!("../sql/drop_indices.sql");
        let drop_views_sql = include_str!("../sql/drop_views.sql");

        // Drop views first
        db.execute_unprepared(drop_views_sql).await?;

        // Drop indices
        db.execute_unprepared(drop_indices_sql).await?;

        Ok(())
    }
}