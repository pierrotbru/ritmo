use sqlx::SqlitePool;
use crate::errors::RitmoErr;

mod seeding_contents_types;
mod seeding_languages;
mod seeding_roles;
mod seeding_laverdure;

use seeding_contents_types::seed_content_types;
use seeding_languages::seed_languages_names_table;
use seeding_roles::seed_roles;
use seeding_laverdure::seed_laverdure_table;


pub async fn seed_all(pool: &SqlitePool) -> Result<(), RitmoErr> {
  
    let _ = seed_languages_names_table(pool).await?;
    let _ = seed_laverdure_table(pool).await?;
    let _ = seed_content_types(pool).await?;
    let _ = seed_roles(pool).await?;

    Ok(())
}
