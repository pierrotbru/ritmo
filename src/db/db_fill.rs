use crate::errors::RitmoErr;
use sqlx::{Executor, SqlitePool};
use tracing::{info, error};

pub async fn fill_db(pool: &SqlitePool) -> Result<(), RitmoErr> {

    info!("Avvio popolamento database");

    let mut tx = pool.begin().await.map_err(|e| {
        error!("Errore inizio transazione: {}", e);
        RitmoErr::DatabaseCreationFailed(format!("Avvio transazione fallito: {}", e))
    })?;

    tx.execute("
        INSERT OR REPLACE INTO laverdure (key, value)
        VALUES
            ('author', 'laverdure'),
            ('program', 'ritmo'),
            ('program release', '1.0.0'),
            ('database_version', '1.0.0')
    ").await.map_err(|e| {
        error!("Errore inserimento metadati: {}", e);
        RitmoErr::DatabaseCreationFailed(format!("Inserimento metadati fallito: {}", e))
    })?;

    info!("Metadati inseriti");

    // Insert default roles
    let roles = vec!["Author", "Translator", "Curator"];
    for role in roles {
        match sqlx::query("INSERT OR IGNORE INTO Role (role_name) VALUES (?)")
            .bind(role)
            .execute(&mut *tx)
            .await {
                Ok(_) => info!("Inserted role: {}", role),
                Err(e) => {
                    error!("Failed to insert role {}: {}", role, e);
                    return Err(RitmoErr::DatabaseCreationFailed(format!("Role insertion failed for {}: {}", role, e)));
                }
            }
    }

    tx.commit().await.map_err(|e| {
        error!("Errore commit transazione: {}", e);
        RitmoErr::DatabaseCreationFailed(format!("Commit transazione fallito: {}", e))
    })?;

    info!("Popolamento database completato");
    Ok(())
}

