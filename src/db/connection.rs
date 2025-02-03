// src/db/connection.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ritmo.db".to_string());
    
    Database::connect(&database_url).await
}