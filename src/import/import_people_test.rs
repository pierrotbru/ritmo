use crate::db::enhanced_name_manager_2::MatchResult;
use crate::db::enhanced_name_manager_2::NameManager;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::time::Instant;
use crate::RitmoErr;

#[derive(sqlx::FromRow, Debug)]
struct Person {
    id: i64,
    name: String,
}

pub async fn import_people_test(src: &SqlitePool, _dst: &SqlitePool) -> Result<(), RitmoErr> {
    let _start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM authors")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch rows for table authors: {}", e)))?;

    let names_from_file: Vec<String> = calibre_rows
        .into_iter()
        .map(|row| {
            row.get::<String, _>("name")
        })
        .collect();

    let mut name_manager = NameManager::new();

    if names_from_file.is_empty() {
        eprintln!("AVVISO: Nessun nome trovato nel file db. Assicurati che il file esista e contenga nomi validi.");
        return Ok(()); // Termina se non ci sono nomi da processare.
    }

    println!("{} nomi letti con successo dal file.", names_from_file.len());

    // 3. Elaborazione e aggiunta dei record di persone al NameManager dal file iniziale.
    // Per ogni nome letto, crea un PersonRecord e lo aggiunge al NameManager.
    // 
    let mut current_id = 0;
    for name_input in names_from_file {
        current_id += 1; // Incrementa l'ID per ogni nuovo record.
        match name_manager.create_person_record(&name_input, current_id) {
            Ok(person_record) => {
                println!("  Creando record iniziale per: '{}' (ID: {})", person_record.original_input, person_record.id);
                name_manager.add_person_record(person_record)?;
            },
            Err(e) => {
                eprintln!("Errore nella creazione del record iniziale per '{}': {}", name_input, e);
            }
        }
    }
    println!("{} record di persone aggiunti al NameManager dall'input iniziale.", name_manager.all_person_records.len());

    // 4. Esecuzione del training del modello ML (con i dati iniziali).
    println!("\nAvvio del training del modello ML...");
    name_manager.train_ml_model()?; // Esegue il training del modello ML.

    // 5. Inserimento di nuovi nomi con controllo dei duplicati.
    println!("\nInizio inserimento di nuovi nomi con controllo duplicati...");
    let names_to_insert = vec![
        "Giuseppe Verdi".to_string(), // Probabilmente già presente se "Giuseppe" è nel file
        "Mario Rossi".to_string(),    // Probabilmente già presente se "Mario" è nel file
        "Luca Bianchini".to_string(), // Nome che non dovrebbe essere un duplicato esatto.
        "Luka Bianchini".to_string(), // Una possibile variante/typo di "Luca Bianchini"
        "Francesco D'Agostino".to_string(), // Un altro nuovo nome
        "Fra D'Agostino".to_string(), // Un'abbreviazione
        "Giovanni Bacci".to_string(), // Nuovo
        "Giovani Baci".to_string(), // Variante fonetica/typo
        "Rossi, Mario".to_string(),
        "Rossi Mario".to_string(),
        "asa larsson".to_string(),
        "larsson, asa".to_string()
    ];

    for new_name_input in names_to_insert {
        println!("\nProcessando nome: '{}'", new_name_input);

        // Prima di aggiungere, cerca match esistenti
        let match_result = name_manager.find_matches(&new_name_input);

        match match_result {
            MatchResult::ExactMatch(id) => {
                println!("  -> MATCH ESATTO trovato per '{}' con ID: {}. Non verrà aggiunto come nuovo record.", new_name_input, id);
            },
            MatchResult::HighConfidenceMatch(matches) => {
                println!("  -> MATCH AD ALTA CONFIDENZA trovato per '{}':", new_name_input);
                for m in matches {
                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
                }
                println!("  Non verrà aggiunto come nuovo record, potrebbe essere una variante/duplicato molto simile.");
                // Qui potresti decidere di aggiornare il record esistente con un alias
                // o fare altre azioni basate sull'alta confidenza.
                // Esempio (per scopi dimostrativi):
                // let base_name = name_manager.all_person_records.get(&matches[0].person_id).map(|p| p.normalized_key.clone());
                // if let Some(base) = base_name {
                //    name_manager.incremental_learning(vec![(base, new_name_input.clone(), 0.99)])?;
                // }
            },
            MatchResult::PossibleMatches(matches) => {
                println!("  -> POSSIBILI MATCH trovati per '{}':", new_name_input);
                for m in matches {
                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
                }
                println!("  Considerando l'aggiunta come nuovo record, ma tieni d'occhio questi possibili match.");
                // In questo caso, puoi decidere di aggiungere il record ma magari con un flag di "da revisionare".
                add_new_record(&mut name_manager, &mut current_id, &new_name_input)?;
            },
            MatchResult::NoMatch => {
                println!("  -> NESSUN MATCH trovato per '{}'. Aggiungerò come nuovo record.", new_name_input);
                add_new_record(&mut name_manager, &mut current_id, &new_name_input)?;
            },
        }
    }

    println!("\nProcesso di inserimento completato.");
    println!("Numero finale di record nel gestore: {}", name_manager.all_person_records.len());
    println!("\nApplicazione terminata con successo.");

    // --- Esempio di unificazione manuale dei record ---
    // Questi ID sono fittizi e dovrebbero essere identificati dalla tua logica
    // o tramite revisione manuale dell'output del log.
    // Assumiamo che ID 1 e ID 2 siano Giuseppe Verdi e Giusepe Verdi che vogliamo unire.
    println!("\n--- Simulazione di unificazione manuale dei record ---");
    let initial_record_count = name_manager.all_person_records.len();
    if initial_record_count >= 2 { // Assicurati che ci siano almeno due record per la fusione.
        // ID esempio per dimostrazione. Nella realtà, li identificheresti dopo un'analisi.
        let keeper_id = 1; // ID del record da mantenere
        let duplicate_id = 2; // ID del record da unire e rimuovere

        // Verifica che gli ID esistano prima di tentare la fusione
        if name_manager.all_person_records.contains_key(&keeper_id) && name_manager.all_person_records.contains_key(&duplicate_id) {
            println!("Tentativo di unire record con ID {} nel record con ID {}", duplicate_id, keeper_id);
            match name_manager.merge_person_records(keeper_id, duplicate_id) {
                Ok(_) => {
                    println!("Unificazione completata con successo.");
                    println!("Numero finale di record dopo l'unificazione: {}", name_manager.all_person_records.len());
                },
                Err(e) => {
                    eprintln!("Errore durante l'unificazione: {}", e);
                }
            }
        } else {
            println!("Impossibile simulare l'unificazione: gli ID specificati non esistono nel gestore.");
        }
    } else {
        println!("Numero insufficiente di record per simulare l'unificazione.");
    }
    println!("--- Fine simulazione unificazione ---");

    Ok(())
}

// Funzione ausiliaria per aggiungere un nuovo record
fn add_new_record(
    name_manager: &mut NameManager,
    current_id: &mut i64,
    name_input: &str,
) -> Result<(), RitmoErr> {
    *current_id += 1; // Incrementa l'ID per il nuovo record
    match name_manager.create_person_record(name_input, *current_id) {
        Ok(new_record) => {
            println!("    Aggiungendo nuovo record in memoria per '{}' con ID: {}", new_record.parsed_name.display_name, new_record.id);
            name_manager.add_person_record(new_record)?;
        },
        Err(e) => {
            eprintln!("    Errore nella creazione del record per '{}': {}", name_input, e);
        }
    }
    Ok(())
}