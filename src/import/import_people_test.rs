use crate::names::NameManager;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::time::Instant;
use crate::RitmoErr;

//pub async fn import_people_test(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {
//    let _start = Instant::now();
//
//    let mut name_manager = NameManager::new();
//    name_manager.load_names_from_db(dst).await?;
//    if name_manager.all_person_records.len() > 0 {
//        // qui carico tutti i dati di ML
//        name_manager.ml_learner = MLNameLearner::load_from_db(dst).await?;
//        name_manager.name_utils = NameUtils::load_from_db(dst).await?;
//    }
//
//    let names_from_file: Vec<(i64, String)> = sqlx::query("SELECT id, name FROM authors")
//        .fetch_all(src)
//        .await
//        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table authors: {}", e)))?
//        .into_iter()
//        .map(|row| {
//            (row.get::<i64, _>("id"), row.get::<String, _>("name"))
//        })
//        .collect();
//
//
//    if names_from_file.is_empty() {
//        eprintln!("AVVISO: Nessun nome trovato nel file db. Assicurati che il file esista e contenga nomi validi.");
//        return Ok(()); // Termina se non ci sono nomi da processare.
//    }
//
//
//
//
//    for (mut i, new_name_input) in names_from_file.iter().enumerate() {
//        i = i+1;
//        if i%500 == 0 {
//            name_manager.train_ml_model()?; // Esegue il training del modello ML.
//        }
//        let match_result = name_manager.find_matches(&new_name_input.1);
//
//        match match_result {
//            MatchResult::ExactMatch(id) => {
//                match name_manager.all_person_records.get(&id) {
//                    Some(person_record) => {
//                        println!("  -> MATCH ESATTO trovato per '{}' con ID: {}, Nome esistente: '{}'. Non verrà aggiunto come nuovo record.", 
//                                 new_name_input.1, id, person_record.original_input);
//                    },
//                    None => {
//                        println!("  -> MATCH ESATTO trovato per '{}' con ID: {}, ma record non trovato. Non verrà aggiunto come nuovo record.", 
//                                 new_name_input.1, id);
//                    }
//                }
//            },
//            MatchResult::HighConfidenceMatch(matches) => {
//                println!("  -> MATCH AD ALTA CONFIDENZA trovato per '{}':", new_name_input.1);
//                for m in matches {
//                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
//                }
//                println!("  Non verrà aggiunto come nuovo record, potrebbe essere una variante/duplicato molto simile.");
//                // Qui potresti decidere di aggiornare il record esistente con un alias
//                // o fare altre azioni basate sull'alta confidenza.
//                // Esempio (per scopi dimostrativi):
//                // let base_name = name_manager.all_person_records.get(&matches[0].person_id).map(|p| p.normalized_key.clone());
//                // if let Some(base) = base_name {
//                //    name_manager.incremental_learning(vec![(base, new_name_input.clone(), 0.99)])?;
//                // }
//            },
//            MatchResult::PossibleMatches(matches) => {
//                println!("  -> POSSIBILI MATCH trovati per '{}':", new_name_input.1);
//                for m in matches {
//                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
//                }
//                println!("  Considerando l'aggiunta come nuovo record, ma tieni d'occhio questi possibili match.");
//                // In questo caso, puoi decidere di aggiungere il record ma magari con un flag di "da revisionare".
//                name_manager.add_new_record(new_name_input.0, &new_name_input.1)?;
//            },
//            MatchResult::NoMatch => {
////                println!("  -> NESSUN MATCH trovato per '{}'. Aggiungerò come nuovo record.", new_name_input);
//                name_manager.add_new_record(new_name_input.0, &new_name_input.1)?;
//            },
//        }
//    }
//
//    println!("\nProcesso di inserimento completato.");
//    println!("Numero finale di record nel gestore: {}", name_manager.all_person_records.len());
//    println!("\nApplicazione terminata con successo.");
//
//    name_manager.save_manager_person_records_to_db(dst).await?;
////    name_manager.save_ml_data_to_db(dst).await?;
//    Ok(())
//}
//
//// Funzione ausiliaria per aggiungere un nuovo record
//fn add_new_record(
//    name_manager: &mut NameManager,
//    current_id: i64,
//    name_input: &str,
//) -> Result<(), RitmoErr> {
//    match name_manager.create_person_record(name_input, current_id) {
//        Ok(new_record) => {
////            println!("    Aggiungendo nuovo record in memoria per '{}' con ID: {}", new_record.parsed_name.display_name, new_record.id);
//            name_manager.add_person_record(new_record)?;
//        },
//        Err(e) => {
//            eprintln!("    Errore nella creazione del record per '{}': {}", name_input, e);
//        }
//    }
//
//    Ok(())
//}
//
pub async fn import_people_test(src_pool: &SqlitePool, dst_pool: &SqlitePool) -> Result<(), RitmoErr> {
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
    // Carica i record esistenti nel NameManager prima del processamento
    // Questo è CRUCIALE per permettere a find_matches di funzionare correttamente
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
    // L'intervallo per il training ML è passato qui, ad esempio ogni 500 nomi.
    let new_ids_added = (&mut name_manager).process_names_with_matching_and_ml_training(dst_pool, names_for_processing).await?;

    println!("\nProcesso di importazione completato.");
    println!("{} nuovi record sono stati aggiunti (o gli ID originali sono stati mantenuti se non duplicati).", new_ids_added.len());
    println!("Numero finale di record nel gestore: {}", name_manager.all_person_records.len());
    println!("\nApplicazione terminata con successo.");

    // Assicurati che il NameManager sia persistito correttamente nel DB
    // name_manager.save_ml_data_to_db(dst_pool).await?; // Se hai dati ML separati
    Ok(())
}