// src/name_matching/process.rs

use sqlx::SqlitePool;
use std::time::Instant;
use crate::errors::RitmoErr;
use super::manager::NameManager;
use super::models::{PersonRecord, MatchResult}; // Importa i tipi necessari

impl NameManager {
    /// Processa una lista di nomi, applicando la logica di matching e aggiungendo i record
    /// al NameManager e al database. Gli ID possono essere forniti (per importazioni)
    /// o generati (per nuovi nomi). Include il training ML a intervalli regolari.
    ///
    /// Ritorna un vettore contenente gli IDs dei nomi che sono stati aggiunti come nuovi record.
    pub async fn process_names_with_matching_and_ml_training(
        &mut self, pool: &SqlitePool,
        names_to_process: Vec<(Option<i64>, String)>, // (ID opzionale, Nome)
    ) -> Result<Vec<i64>, RitmoErr> {
        let mut new_person_records_to_add_buffer: Vec<PersonRecord> = Vec::new(); // Buffer per i record da salvare nel DB
        let mut added_ids_this_run: Vec<i64> = Vec::new(); // IDs dei record effettivamente aggiunti come nuovi

        const INTERVALS: [i32; 6] = [ 100, 200, 500, 1000, 2000, 5000];

        println!("Inizio processamento di {} nomi...", names_to_process.len());
        let start_time = Instant::now();

        // Carica tutti i record esistenti nel NameManager all'inizio.
        // Questo è cruciale per find_matches e per mantenere il conteggio degli ID.
        self.load_names_from_db(pool).await?;
        println!("{} record esistenti caricati nel NameManager.", self.all_person_records.len());
        if self.all_person_records.len() > 0 {
            self.load_ml_from_db(pool).await?;
        }


        for (provided_id, name_input) in names_to_process {

            let n = (self.all_person_records.len() + added_ids_this_run.len()) as i32;
            let last = *INTERVALS.iter().max().unwrap();
            if INTERVALS.contains(&n) || n > last && n % last == 0 {
                self.train_ml_model()?;
            }

            let match_result = self.find_matches(&name_input);
            let mut should_add_new = false;

            match match_result {
                MatchResult::ExactMatch(id) => {
                    println!("  -> MATCH ESATTO trovato per '{}' con ID: {}. Non verrà aggiunto come nuovo record.", name_input, id);
                },
                MatchResult::HighConfidenceMatch(matches) => {
                    if matches[0].confidence >= 0.95 {
                        println!("  -> MATCH AD ALTA CONFIDENZA trovato per '{}' (ID: {}, Confidenza: {:.2}). Non verrà aggiunto come nuovo record.",
                                 name_input, matches[0].person_id, matches[0].confidence);
                    } else {
                        should_add_new = true;
                        println!("  -> MATCH AD ALTA CONFIDENZA (ma confidenza < 0.95) per '{}'. Considerando l'aggiunta.", name_input);
                    }
                },
                MatchResult::PossibleMatches(matches) => {
                    should_add_new = true;
                    println!("  -> POSSIBILI MATCH trovati per '{}'. Aggiungerò come nuovo record, tieni d'occhio questi possibili match.", name_input);
                    for m in matches {
                        println!("    - ID: {}, Nome: '{}', Score: {:.2}", m.person_id, m.matched_name, m.similarity_score);
                    }
                },
                MatchResult::NoMatch => {
                    should_add_new = true;
//                    println!("  -> NESSUN MATCH trovato per '{}'. Aggiungerò come nuovo record.", name_input);
                },
            }

            if should_add_new {
                match self.create_person_record_with_id(&name_input, provided_id) {
                    Ok(new_record) => {
                        let record_id = new_record.id;
                        self.add_person_record(new_record.clone())?;
                        new_person_records_to_add_buffer.push(new_record);
                        added_ids_this_run.push(record_id);
                    },
                    Err(e) => {
                        eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
                    }
                }
            }
        }

        if !new_person_records_to_add_buffer.is_empty() {
            println!("Salvataggio finale di {} record nel DB...", new_person_records_to_add_buffer.len());
            self.save_person_records_to_db(pool, &new_person_records_to_add_buffer).await?;
            new_person_records_to_add_buffer.clear();
            println!("Salvataggio finale completato.");
        }

        let elapsed_time = start_time.elapsed();
        println!("\nProcessamento completato in {:?}", elapsed_time);
        println!("Totale record aggiunti come nuovi: {}", added_ids_this_run.len());
        println!("Numero finale di record nel gestore: {}", self.all_person_records.len());
        println!("\nApplicazione terminata con successo.");

        Ok(added_ids_this_run)
    }
}
