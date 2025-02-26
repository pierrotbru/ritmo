use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;

pub async fn seed_roles(pool: &SqlitePool) -> Result<(), RitmoErr> {
    let types_data = vec![
        "Author",
        "Translator",
        "Curator",
        "Cover designer",
        "Commentator",
        "Interviewer",
        "Reporter",
        "Photographer",
        "Comic book artist",
    ];

    let start = Instant::now();

    let mut tx = pool.begin().await.map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    for name in types_data {
        let _ = query!("INSERT INTO roles (name) VALUES (?)", name)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    tx.commit().await.map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

    let duration = start.elapsed();
    println!("sqlx seeding roles (transaction): {:?}", duration);
    Ok(())
}
