// src/name_matching/manager.rs

use sqlx::SqlitePool;
use crate::names::MatchResult;
use crate::names::names_ml::MLNameLearner;
use std::collections::{HashMap, HashSet};
use rphonetic::DoubleMetaphone;
use fuzzy_matcher::skim::SkimMatcherV2;
use crate::errors::RitmoErr;
use super::models::PersonRecord;
use super::utils::NameUtils; // Importa le utilità

#[allow(dead_code)]
pub struct NameManager {
    pub fuzzy_matcher: SkimMatcherV2,
    pub name_utils: NameUtils, // Incapsuliamo qui le utilità
    pub common_abbreviations: HashMap<String, Vec<String>>,
    pub similarity_threshold: f64,
    pub typo_threshold: f64,
    pub all_person_records: HashMap<i64, PersonRecord>,
    pub normalized_key_index: HashMap<String, HashSet<i64>>,
    pub phonetic_key_index: HashMap<String, HashSet<i64>>,
    pub name_variants_internal: HashMap<String, Vec<String>>, // Spostato qui per NameUtils
    pub ml_learner: MLNameLearner,
}

impl NameManager {
    pub fn new() -> Self {
        let mut common_abbreviations = HashMap::new();
        common_abbreviations.insert("giuseppe".to_string(), vec!["peppe".to_string(), "beppe".to_string()]);
        common_abbreviations.insert("giovanni".to_string(), vec!["gianni".to_string(), "gian".to_string()]);
        common_abbreviations.insert("francesco".to_string(), vec!["franco".to_string(), "checco".to_string()]);

        let mut name_variants_internal = HashMap::new(); // Inizializza qui
        name_variants_internal.insert("anton".to_string(), vec!["antonio".to_string(), "antony".to_string()]);
        name_variants_internal.insert("pavlovic".to_string(), vec!["pavlociv".to_string(), "pavlovič".to_string()]);
        name_variants_internal.insert("cechov".to_string(), vec!["chekhov".to_string(), "čechov".to_string(), "tchekhov".to_string()]);
        name_variants_internal.insert("franc".to_string(), vec!["frank".to_string(), "franck".to_string(), "francesco".to_string()]);

        let double_metaphone = DoubleMetaphone::default();
        let name_utils = NameUtils::new(double_metaphone, name_variants_internal.clone()); // Passa le varianti per NameUtils

        Self {
            fuzzy_matcher: SkimMatcherV2::default(),
            name_utils,
            common_abbreviations,
            similarity_threshold: 0.75,
            typo_threshold: 0.85,
            all_person_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            phonetic_key_index: HashMap::new(),
            name_variants_internal, // Conserva una copia per NameManager stesso se necessario, o gestisci solo tramite NameUtils
            ml_learner: MLNameLearner::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_person_name_by_id(&self, id: i64) -> Option<String> {
        self.all_person_records
            .get(&id)
            .map(|record| record.original_input.clone())
    }

    #[allow(dead_code)]
    pub fn merge_person_records(&mut self, keeper_id: i64, duplicate_id: i64) -> Result<(), RitmoErr> {
        if keeper_id == duplicate_id {
            return Err(RitmoErr::MergeError("Impossibile unire un record con se stesso.".to_string()));
        }

        let duplicate_record = self.all_person_records.remove(&duplicate_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record duplicato con ID {} non trovato.", duplicate_id)))?;

        let keeper_record = self.all_person_records.get_mut(&keeper_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record principale con ID {} non trovato.", keeper_id)))?;

        // Logic for merging aliases (already good)
        if !keeper_record.aliases.contains(&duplicate_record.original_input) &&
           keeper_record.original_input != duplicate_record.original_input {
            keeper_record.aliases.push(duplicate_record.original_input.clone());
        }

        for alias in duplicate_record.aliases {
            if !keeper_record.aliases.contains(&alias) &&
               keeper_record.original_input != alias {
                keeper_record.aliases.push(alias);
            }
        }

        // Update indices for the merged record
        let duplicate_normalized_key = duplicate_record.normalized_key.clone();
        if let Some(ids) = self.normalized_key_index.get_mut(&duplicate_normalized_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            if ids.is_empty() {
                self.normalized_key_index.remove(&duplicate_normalized_key);
            }
        }

        let duplicate_phonetic_key = duplicate_record.phonetic_key.clone();
        if let Some(ids) = self.phonetic_key_index.get_mut(&duplicate_phonetic_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            if ids.is_empty() {
                self.phonetic_key_index.remove(&duplicate_phonetic_key);
            }
        }
        Ok(())
    }

    // Metodo per aggiungere un alias a un record esistente
    pub fn add_alias_to_person_record(&mut self, person_id: i64, alias_name: String) -> Result<(), RitmoErr> {
        if let Some(record) = self.all_person_records.get_mut(&person_id) {
            // Aggiungi l'alias solo se non esiste già
            if !record.aliases.contains(&alias_name) && record.original_input != alias_name {
                 record.aliases.push(alias_name.clone()); // Aggiungi l'alias alla lista del record
                 // Aggiorna gli indici per l'alias appena aggiunto
                 let normalized_alias = self.name_utils.normalize_string(&alias_name);
                 let phonetic_alias = self.name_utils.generate_phonetic_key(&alias_name);

                 self.normalized_key_index.entry(normalized_alias)
                     .or_default()
                     .insert(person_id);

                 self.phonetic_key_index.entry(phonetic_alias)
                     .or_default()
                     .insert(person_id);
                 Ok(())
            } else {
                Ok(()) // L'alias è già presente o è uguale al nome primario
            }
        } else {
            Err(RitmoErr::DatabaseQueryFailed(format!("PersonRecord con ID {} non trovato per aggiungere alias.", person_id)))
        }
    }


    #[allow(dead_code)]
    pub fn add_new_record(&mut self, current_id: i64, name_input: &str) -> Result<(), RitmoErr> {
        match self.create_person_record(name_input, current_id) {
            Ok(new_record) => {
                self.add_person_record(new_record)?;
            },
            Err(e) => {
                eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
            }
        }
        Ok(())
    }

    pub fn add_person_record(&mut self, record: PersonRecord) -> Result<(), RitmoErr> {
        let id = record.id;
        let normalized_key = record.normalized_key.clone();
        let phonetic_key = record.phonetic_key.clone();

        self.all_person_records.insert(id, record.clone());

        self.normalized_key_index.entry(normalized_key)
            .or_default()
            .insert(id);

        self.phonetic_key_index.entry(phonetic_key)
            .or_default()
            .insert(id);

        for alias in &record.aliases {
            let normalized_alias = self.name_utils.normalize_string(alias);
            let phonetic_alias = self.name_utils.generate_phonetic_key(alias);

            self.normalized_key_index.entry(normalized_alias)
                .or_default()
                .insert(id);

            self.phonetic_key_index.entry(phonetic_alias)
                .or_default()
                .insert(id);
        }
        Ok(())
    }

    // Metodo per generare il prossimo ID UNICO disponibile
    pub fn generate_next_id(&mut self) -> i64 {
        let next_id = self.all_person_records.keys().max().copied().unwrap_or(0) + 1;
        next_id
    }

    pub fn create_person_record(&self, input: &str, id: i64) -> Result<PersonRecord, RitmoErr> {
        let parsed_name = self.name_utils.parse_name(input)?;
        let normalized_key = self.name_utils.normalize_parsed_name_for_matching(&parsed_name);
        let phonetic_key = self.name_utils.generate_phonetic_key(&normalized_key);

        Ok(PersonRecord {
            id,
            original_input: input.to_string(),
            parsed_name,
            normalized_key,
            phonetic_key,
            confidence: 1.0,
            verified: false,
            aliases: Vec::new(),
        })
    }

    /// Crea un nuovo PersonRecord con l'ID fornito o generandone uno se `None`.
    /// Questo metodo incapsula la logica di incremento dell'ID e creazione del record.
    pub fn create_person_record_with_id(&mut self, name_input: &str, id: Option<i64>) -> Result<PersonRecord, RitmoErr> {
        let record_id = if let Some(existing_id) = id {
            existing_id
        } else {
            self.generate_next_id()
        };
        self.create_person_record(name_input, record_id)
    }

    // `add_name_variant` non è più usato direttamente da NameManager, ma da NameUtils
    // o dalla logica ML che poi chiama add_alias_to_person_record se applicabile.
    // Se servesse una gestione centrale di `name_variants_internal` in NameManager,
    // dovresti aggiungere un metodo pubblico qui.
    pub fn add_internal_name_variant(&mut self, base_name: &str, variant: &str) {
        let base_normalized = self.name_utils.normalize_string(base_name);
        let variant_normalized = self.name_utils.normalize_string(variant);

        self.name_variants_internal
            .entry(base_normalized.clone())
            .or_default()
            .push(variant_normalized.clone());

        self.name_variants_internal
            .entry(variant_normalized)
            .or_default()
            .push(base_normalized);
    }

    #[allow(dead_code)]
    /// Aggiunge al database i nomi contenuti nel vettore in ingresso, ed inoltre li aggiunge anche a self.
    /// Ritorna un vettore contenente gli IDs dei nomi inseriti.
    /// Accetta una tupla di (ID opzionale, nome) per permettere l'inserimento con ID predefiniti.
    pub async fn process_and_add_names_unified(&mut self, pool: &SqlitePool, names_to_process: Vec<(Option<i64>, String)>) -> Result<Vec<i64>, RitmoErr> {
        let mut new_person_records_to_add: Vec<PersonRecord> = Vec::new();
        let mut new_person_ids: Vec<i64> = Vec::new();

        // current_max_id non è più necessario all'inizio del ciclo, perché create_person_record_with_id lo gestisce.

        for (provided_id, name_input) in names_to_process {
            let match_result = self.find_matches(&name_input);
            let should_add_new = match match_result {
                MatchResult::ExactMatch(_) => false,
                MatchResult::HighConfidenceMatch(matches) => {
                    if matches[0].confidence >= 0.95 {
                        false
                    } else {
                        true
                    }
                },
                MatchResult::PossibleMatches(_) | MatchResult::NoMatch => true,
            };

            if should_add_new {
                match self.create_person_record_with_id(&name_input, provided_id) {
                    Ok(new_record) => {
                        new_person_records_to_add.push(new_record.clone()); // Clone perché ne hai bisogno dopo
                        new_person_ids.push(new_record.id); // Aggiungi l'ID del nuovo record
                    },
                    Err(e) => {
                        eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
                    }
                }
            }
        }

        if !new_person_records_to_add.is_empty() {
            for record in &new_person_records_to_add {
                self.add_person_record(record.clone())?; // Aggiungi al NameManager
            }
            self.save_person_records_to_db(pool, &new_person_records_to_add).await?;
        }
        return Ok(new_person_ids);
    }
}