use crate::db::seeding::seed_all;
use sqlx::Executor;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::fs;

use crate::errors::RitmoErr;
use crate::db::verify_path::verify_path;



pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    // call verify_path to check if the db_path is valid
    let db_path = verify_path(path, create)?;

    if create {
        fs::File::create(db_path.clone())?;
    }

    // Construct the database URL with explicit SQLite driver
    let database_url = format!("sqlite:///{}", db_path.to_string_lossy());
    
    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations if create is true
    if create {
        pool.execute(include_str!("./create_sql/create_db.sql"))
        .await?;

        seed_all(&pool).await?;
    }

    Ok(pool)
}
