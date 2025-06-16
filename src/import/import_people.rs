use crate::names::NameManager;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::time::Instant;
use crate::RitmoErr;

pub async fn import_people(src_pool: &SqlitePool, dst_pool: &SqlitePool) -> Result<(), RitmoErr> {
    let _start = Instant::now();

    let names_from_file: Vec<(i64, String)> = sqlx::query("SELECT id, name FROM authors")
        .fetch_all(src_pool)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table authors: {}", e)))?
        .into_iter()
        .map(|row| {
            (row.get::<i64, _>("id"), row.get::<String, _>("name"))
        })
        .collect();

    let mut name_manager = NameManager::new();
    name_manager.load_names_from_db(dst_pool).await?;

    if names_from_file.is_empty() {
        eprintln!("AVVISO: Nessun nome trovato nel file db. Assicurati che il file esista e contenga nomi validi.");
        return Ok(());
    }

    println!("{} nomi letti con successo dal file per l'importazione.", names_from_file.len());

    // Prepara i nomi per la funzione unificata, includendo gli ID originali
    let names_for_processing: Vec<(Option<i64>, String)> = names_from_file.into_iter()
        .map(|(id, name)| (Some(id), name)) // Mappa gli ID esistenti in Some(id)
        .collect();

    // Chiama la funzione unificata per processare tutti i nomi.
    let new_ids_added = (&mut name_manager).process_names_with_matching_and_ml_training(dst_pool, names_for_processing).await?;

    println!("\nProcesso di importazione completato.");
    println!("{} nuovi record sono stati aggiunti (o gli ID originali sono stati mantenuti se non duplicati).", new_ids_added.len());
    println!("Numero finale di record nel gestore: {}", name_manager.all_person_records.len());
    println!("\nApplicazione terminata con successo.");

    // Assicurati che il NameManager sia persistito correttamente nel DB
    // name_manager.save_ml_data_to_db(dst_pool).await?; // Se hai dati ML separati
    Ok(())
}