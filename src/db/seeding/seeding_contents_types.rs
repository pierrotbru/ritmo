use std::time::Instant;
use crate::RitmoErr;
use sqlx::{sqlite::SqlitePool, query};

pub async fn seed_content_types(pool: &SqlitePool) -> Result<(), RitmoErr> {
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

    let start = Instant::now();

    // Inizia la transazione
    let mut tx = pool.begin().await.map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    for name in types_data {
        let _ = query!("INSERT INTO types (name) VALUES (?)", name)
            .execute(&mut *tx) // Usa la transazione
            .await;
        }

    // Commit della transazione
    tx.commit().await.map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

    let duration = start.elapsed();
    println!("sqlx seeding types (transaction): {:?}", duration);
    Ok(())
}
