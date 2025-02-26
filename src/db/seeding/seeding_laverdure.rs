use sqlx::SqlitePool;
use sqlx::query;

use std::time::Instant;

use crate::RitmoErr;

pub async fn seed_laverdure_table(pool: &SqlitePool) -> Result<(), RitmoErr> {
    let laverdure_data = vec![
        ("author", "laverdure"),
        ("program", "ritmo"),
        ("program release", "0.0.0"),
        ("database_version", "0.0.0"),
    ];

    let start = Instant::now();
    let mut tx = pool.begin().await.map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    for (key, value) in laverdure_data {
        let _ = query!("INSERT INTO laverdure (key, value) VALUES (?, ?)", key, value)
            .execute(&mut *tx) // Usa la transazione
            .await;
        }

    // Commit della transazione
    tx.commit().await.map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

    let duration = start.elapsed();
    println!("sqlx seeding laverdure: {:?}", duration);
    Ok(())
}
