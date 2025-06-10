use crate::db::names_ml::MLNameLearner;
use sqlx::{Row, Transaction, Sqlite, SqlitePool, query};
use human_name::Name;
use strsim::{jaro_winkler, levenshtein};
use fuzzy_matcher::skim::SkimMatcherV2;
use unicode_normalization::UnicodeNormalization;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use rphonetic::{DoubleMetaphone, Encoder};
use crate::errors::RitmoErr;

#[derive(Debug)]
pub enum NameManagerErrorInternal {
    NameParsingError(String),
}

impl std::fmt::Display for NameManagerErrorInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NameManagerErrorInternal::NameParsingError(msg) => write!(f, "Errore di parsificazione nome interno: {}", msg),
        }
    }
}

impl Error for NameManagerErrorInternal {}

impl From<NameManagerErrorInternal> for RitmoErr {
    fn from(err: NameManagerErrorInternal) -> Self {
        match err {
            NameManagerErrorInternal::NameParsingError(msg) => RitmoErr::NameParsingError(msg),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonRecord {
    pub id: i64,
    pub original_input: String,
    pub parsed_name: ParsedName,
    pub normalized_key: String,
    pub phonetic_key: String, // NUOVO: chiave fonetica
    pub confidence: f64,
    pub verified: bool,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ParsedName {
    pub given_name: String,
    pub surname: String,
    pub middle_names: Vec<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub display_name: String,
}

#[derive(Debug)]
pub enum MatchResult {
    ExactMatch(i64),
    HighConfidenceMatch(Vec<NameMatch>),
    PossibleMatches(Vec<NameMatch>),
    NoMatch,
}

#[derive(Debug, Clone)]
pub struct NameMatch {
    pub person_id: i64,
    pub matched_name: String,
    pub similarity_score: f64,
    pub match_type: MatchType,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    Exact,
    NameOrder,       // Mario Rossi vs Rossi Mario
    Phonetic,        // Asimov vs Azimov
    Abbreviated,     // J.R.R. Tolkien vs John Ronald Reuel Tolkien
    Typo,            // Asimof vs Asimov
    Alias,           // Bob vs Robert
    PhoneticSimilar, // NUOVO: matching fonetico
    TypoMinor,       // NUOVO: typo singoli
    TypoMajor,       // NUOVO: typo multipli
    Learned,         // NUOVO: variante appresa dal ML
}

// NUOVO: Strutture per il machine learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameVariantPattern {
    pub base_form: String,
    pub variant_form: String,
    pub pattern_type: VariantPatternType,
    pub confidence: f64,
    pub frequency: usize,
    pub phonetic_similarity: f64,
    pub edit_distance: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum VariantPatternType {
    Suffix,        // Antonio → Anton
    Prefix,        // Giuseppe → Beppe  
    Phonetic,      // Cechov → Chekhov
    Transliteration, // Павлович → Pavlovic
    Abbreviation,  // Francesco → Franco
    Compound,      // Jean-Pierre → Gianpiero
}

#[derive(Debug, Clone)]
pub struct NameCluster {
    pub cluster_id: usize,
    pub members: Vec<String>,
    pub centroid: String,
    pub phonetic_signature: String,
    pub confidence: f64,
}

pub struct NameManager {
    fuzzy_matcher: SkimMatcherV2,
    double_metaphone: DoubleMetaphone, // NUOVO: encoder fonetico
    common_abbreviations: HashMap<String, Vec<String>>,
    similarity_threshold: f64,
    typo_threshold: f64, // NUOVO: soglia per typo
    pub all_person_records: HashMap<i64, PersonRecord>,
    normalized_key_index: HashMap<String, HashSet<i64>>,
    phonetic_key_index: HashMap<String, HashSet<i64>>, // NUOVO: indice fonetico
    name_variants: HashMap<String, Vec<String>>, // NUOVO: varianti conosciute
    ml_learner: MLNameLearner,
}

impl NameManager {
    pub fn new() -> Self {
        let mut common_abbreviations = HashMap::new();
        common_abbreviations.insert("giuseppe".to_string(), vec!["peppe".to_string(), "beppe".to_string()]);
        common_abbreviations.insert("giovanni".to_string(), vec!["gianni".to_string(), "gian".to_string()]);
        common_abbreviations.insert("francesco".to_string(), vec!["franco".to_string(), "checco".to_string()]);
        
        // NUOVO: varianti comuni di nomi
        let mut name_variants = HashMap::new();
        name_variants.insert("anton".to_string(), vec!["antonio".to_string(), "antony".to_string()]);
        name_variants.insert("pavlovic".to_string(), vec!["pavlociv".to_string(), "pavlovič".to_string()]);
        name_variants.insert("cechov".to_string(), vec!["chekhov".to_string(), "čechov".to_string(), "tchekhov".to_string()]);
        name_variants.insert("franc".to_string(), vec!["frank".to_string(), "franck".to_string(), "francesco".to_string()]);
        
        Self {
            fuzzy_matcher: SkimMatcherV2::default(),
            double_metaphone: DoubleMetaphone::default(), // NUOVO
            common_abbreviations,
            similarity_threshold: 0.75, // RIDOTTO da 0.8
            typo_threshold: 0.85, // NUOVO: soglia specifica per typo
            all_person_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            phonetic_key_index: HashMap::new(), // NUOVO
            name_variants, // NUOVO
            ml_learner: MLNameLearner::new(), // NUOVO
        }
    }

    pub fn merge_person_records(&mut self, keeper_id: i64, duplicate_id: i64) -> Result<(), RitmoErr> {
        if keeper_id == duplicate_id {
            return Err(RitmoErr::MergeError("Impossibile unire un record con se stesso.".to_string()));
        }

        // Rimuovi il record duplicato per poter mutare keeper senza problemi di prestito.
        let duplicate_record = self.all_person_records.remove(&duplicate_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record duplicato con ID {} non trovato.", duplicate_id)))?;

        // Ottieni un riferimento mutabile al record principale.
        let keeper_record = self.all_person_records.get_mut(&keeper_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record principale con ID {} non trovato.", keeper_id)))?;

        println!("  Unendo '{}' (ID: {}) in '{}' (ID: {})...",
                 duplicate_record.parsed_name.display_name, duplicate_record.id,
                 keeper_record.parsed_name.display_name, keeper_record.id);

        // Trasferisci l'input originale del duplicato come alias al keeper.
        if !keeper_record.aliases.contains(&duplicate_record.original_input) &&
           keeper_record.original_input != duplicate_record.original_input {
            keeper_record.aliases.push(duplicate_record.original_input.clone());
        }

        // Trasferisci tutti gli alias del duplicato al keeper.
        for alias in duplicate_record.aliases {
            if !keeper_record.aliases.contains(&alias) &&
               keeper_record.original_input != alias { // Evita di aggiungere l'original input del keeper come alias
                keeper_record.aliases.push(alias);
            }
        }

        // Aggiorna gli indici per puntare dal duplicato al keeper.
        // Itera su tutti gli indici e aggiorna l'ID del duplicato con l'ID del keeper.

        // Indice della chiave normalizzata
        let duplicate_normalized_key = duplicate_record.normalized_key.clone();
        if let Some(ids) = self.normalized_key_index.get_mut(&duplicate_normalized_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            // Se il set di ID per questa chiave normalizzata diventa vuoto, rimuovi la chiave.
            if ids.is_empty() {
                self.normalized_key_index.remove(&duplicate_normalized_key);
            }
        }

        // Indice della chiave fonetica
        let duplicate_phonetic_key = duplicate_record.phonetic_key.clone();
        if let Some(ids) = self.phonetic_key_index.get_mut(&duplicate_phonetic_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            // Se il set di ID per questa chiave fonetica diventa vuoto, rimuovi la chiave.
            if ids.is_empty() {
                self.phonetic_key_index.remove(&duplicate_phonetic_key);
            }
        }

        // Qui potresti anche voler aggiornare il database se lo stai usando.
        // Esempio (richiede `sqlx` e `pool`):
        /*
        // Assicurati che il record keeper sia salvato con gli alias aggiornati.
        // Assicurati che il record duplicato sia eliminato.
        // Questo richiederebbe di passare un riferimento al `SqlitePool` o una transazione.
        // Ad esempio:
        // let mut tx = pool.begin().await?;
        // self.save_single_person_record_in_tx(&mut tx, keeper_record).await?; // Aggiorna il keeper
        // sqlx::query!("DELETE FROM people WHERE id = ?", duplicate_id)
        //     .execute(&mut *tx).await?; // Elimina il duplicato
        // tx.commit().await?;
        */

        Ok(())
    }


    // MIGLIORATO: normalizzazione più robusta
    pub fn normalize_string(&self, text: &str) -> String {
        let normalized = text
            .nfc()
            .collect::<String>()
            .to_lowercase()
            // Rimuovi accenti e caratteri speciali
            .chars()
            .map(|c| match c {
                'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
                'è' | 'é' | 'ê' | 'ë' => 'e',
                'ì' | 'í' | 'î' | 'ï' => 'i',
                'ō' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
                'ù' | 'ú' | 'û' | 'ü' => 'u',
                'ç' => 'c',
                'ñ' => 'n',
                'ý' | 'ÿ' => 'y',
                'č' | 'ć' => 'c',
                'š' => 's',
                'ž' => 'z',
                'đ' => 'd',
                'ł' => 'l',
                // Rimuovi caratteri non alfabetici eccetto spazi
                c if c.is_alphabetic() || c.is_whitespace() => c,
                _ => ' ',
            })
            .collect::<String>();
        
        // Rimuovi spazi multipli e trim
        normalized.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    // NUOVO: genera chiave fonetica
    pub fn generate_phonetic_key(&self, text: &str) -> String {
        let normalized = self.normalize_string(text);
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        let mut phonetic_parts = Vec::new();
        
        for part in parts {
//            if let Some(primary) = self.double_metaphone.encode(part).primary {
//                phonetic_parts.push(primary);
//            } else {
//                phonetic_parts.push(part.to_string());
//            }
            phonetic_parts.push(self.double_metaphone.encode(part));
        }
        
        phonetic_parts.join(" ")
    }

    // NUOVO: calcola distanza di Levenshtein normalizzata
    pub fn normalized_levenshtein_distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            return 1.0;
        }
        1.0 - (levenshtein(s1, s2) as f64 / max_len)
    }

    // NUOVO: verifica se due nomi sono varianti conosciute
    pub fn are_known_variants(&self, name1: &str, name2: &str) -> bool {
        let norm1 = self.normalize_string(name1);
        let norm2 = self.normalize_string(name2);
        
        if let Some(variants) = self.name_variants.get(&norm1) {
            if variants.contains(&norm2) {
                return true;
            }
        }
        
        if let Some(variants) = self.name_variants.get(&norm2) {
            if variants.contains(&norm1) {
                return true;
            }
        }
        
        false
    }

    // MIGLIORATO: aggiunta dell'indice fonetico
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
            let normalized_alias = self.normalize_string(alias);
            let phonetic_alias = self.generate_phonetic_key(alias);
            
            self.normalized_key_index.entry(normalized_alias)
                .or_default()
                .insert(id);
                
            self.phonetic_key_index.entry(phonetic_alias)
                .or_default()
                .insert(id);
        }
        
        Ok(())
    }

    // MIGLIORATO: matching più sofisticato
    pub fn find_matches(&self, input_name: &str) -> MatchResult {
        let parsed_input_res = self.parse_name(input_name);
        if parsed_input_res.is_err() {
            return MatchResult::NoMatch;
        }
        
        let parsed_input = parsed_input_res.unwrap();
        let normalized_input = self.normalize_parsed_name_for_matching(&parsed_input);
        let phonetic_input = self.generate_phonetic_key(&normalized_input);
        
        let mut candidate_ids: HashSet<i64> = HashSet::new();
        
        // Cerca nelle chiavi normalizzate
        if let Some(ids) = self.normalized_key_index.get(&normalized_input) {
            candidate_ids.extend(ids);
        }
        
        // NUOVO: cerca nelle chiavi fonetiche
        if let Some(ids) = self.phonetic_key_index.get(&phonetic_input) {
            candidate_ids.extend(ids);
        }
        
        // NUOVO: cerca candidati con similarità alta usando Levenshtein
        for (normalized_key, ids) in &self.normalized_key_index {
            let levenshtein_sim = self.normalized_levenshtein_distance(&normalized_input, normalized_key);
            if levenshtein_sim >= self.typo_threshold {
                candidate_ids.extend(ids);
            }
        }
        
        if candidate_ids.is_empty() {
            return MatchResult::NoMatch;
        }

        let mut matches = Vec::new();
        
        for &person_id in &candidate_ids {
            if let Some(person) = self.all_person_records.get(&person_id) {
                let mut best_match: Option<NameMatch> = None;
                let mut best_score = 0.0;
                
                // 1. Match esatto normalizzato
                let direct_score = jaro_winkler(&normalized_input, &person.normalized_key);
                if direct_score > best_score {
                    best_score = direct_score;
                    let match_type = if direct_score >= 0.99 {
                        MatchType::Exact
                    } else if direct_score >= self.typo_threshold {
                        // NUOVO: distingui tra typo minori e maggiori
                        let levenshtein_sim = self.normalized_levenshtein_distance(&normalized_input, &person.normalized_key);
                        if levenshtein_sim >= 0.9 {
                            MatchType::TypoMinor
                        } else {
                            MatchType::TypoMajor
                        }
                    } else {
                        MatchType::Typo
                    };
                    
                    best_match = Some(NameMatch {
                        person_id: person.id,
                        matched_name: person.parsed_name.display_name.clone(),
                        similarity_score: direct_score,
                        match_type,
                        confidence: direct_score * person.confidence,
                    });
                }

                // 2. NUOVO: Verifica varianti apprese dal ML
                if best_score < 1.0 {
                    if let Some(learned_variant) = self.ml_learner.find_learned_variant(&normalized_input, &person.normalized_key) {
                        best_score = learned_variant.confidence.max(0.88); // Min 88% per varianti apprese
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: learned_variant.confidence,
                            match_type: MatchType::Learned,
                            confidence: learned_variant.confidence * person.confidence,
                        });
                    }
                }

                // 3. Verifica varianti conosciute
                if best_score < 1.0 {
                    if self.are_known_variants(&normalized_input, &person.normalized_key) {
                        best_score = 0.95; // Alta confidenza per varianti conosciute
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: 0.95,
                            match_type: MatchType::Alias,
                            confidence: 0.95 * person.confidence,
                        });
                    }
                }

                // 4. Match fonetico
                if best_score < 1.0 {
                    let phonetic_score = jaro_winkler(&phonetic_input, &person.phonetic_key);
                    if phonetic_score >= 0.8 && phonetic_score > best_score * 0.9 {
                        best_score = phonetic_score * 0.9; // Penalizza leggermente il match fonetico
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: phonetic_score,
                            match_type: MatchType::PhoneticSimilar,
                            confidence: phonetic_score * person.confidence * 0.85,
                        });
                    }
                }

                // 5. Match con nomi invertiti (già esistente, mantieni)
                if best_score < 1.0 {
                    let person_swapped_parsed_name = ParsedName {
                        given_name: person.parsed_name.surname.clone(),
                        surname: person.parsed_name.given_name.clone(),
                        middle_names: person.parsed_name.middle_names.clone(),
                        title: person.parsed_name.title.clone(),
                        suffix: person.parsed_name.suffix.clone(),
                        display_name: format!("{} {}", person.parsed_name.surname, person.parsed_name.given_name),
                    };
                    let person_swapped_normalized_key = self.normalize_parsed_name_for_matching(&person_swapped_parsed_name);
                    let swap_score = jaro_winkler(&normalized_input, &person_swapped_normalized_key);
                    
                    if swap_score > best_score && swap_score >= self.similarity_threshold {
                        best_score = swap_score;
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: swap_score,
                            match_type: MatchType::NameOrder,
                            confidence: swap_score * person.confidence,
                        });
                    }
                }

                // 6. Match con alias (già esistente, mantieni)
                if best_score < 1.0 {
                    for alias in &person.aliases {
                        let alias_score = jaro_winkler(&normalized_input, &self.normalize_string(alias));
                        if alias_score > best_score {
                            best_score = alias_score;
                            best_match = Some(NameMatch {
                                person_id: person.id,
                                matched_name: alias.clone(),
                                similarity_score: alias_score,
                                match_type: MatchType::Alias,
                                confidence: alias_score * person.confidence * 0.9,
                            });
                        }
                    }
                }

                if let Some(m) = best_match {
                    if m.similarity_score >= self.similarity_threshold {
                        matches.push(m);
                    }
                }
            }
        }

        // Ordina per confidenza
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        // Trova match perfetti
        if let Some(perfect_match) = matches.iter().find(|m| m.similarity_score >= 0.99) {
            return MatchResult::ExactMatch(perfect_match.person_id);
        }

        match matches.len() {
            0 => MatchResult::NoMatch,
            _ => {
                let top_match = &matches[0];
                if top_match.confidence > 0.9 {
                    MatchResult::HighConfidenceMatch(matches.into_iter().take(3).collect())
                } else if top_match.confidence > 0.75 {
                    MatchResult::PossibleMatches(matches.into_iter().take(5).collect())
                } else {
                    MatchResult::NoMatch
                }
            }
        }
    }

    // MIGLIORATO: crea record con chiave fonetica
    pub fn create_person_record(&self, input: &str, id: i64) -> Result<PersonRecord, RitmoErr> {
        let parsed_name = self.parse_name(input)?;
        let normalized_key = self.normalize_parsed_name_for_matching(&parsed_name);
        let phonetic_key = self.generate_phonetic_key(&normalized_key);
        
        Ok(PersonRecord {
            id,
            original_input: input.to_string(),
            parsed_name,
            normalized_key,
            phonetic_key, // NUOVO
            confidence: 1.0,
            verified: false,
            aliases: Vec::new(),
        })
    }

    // NUOVO: metodo per aggiungere varianti di nome dinamicamente
    pub fn add_name_variant(&mut self, base_name: &str, variant: &str) {
        let base_normalized = self.normalize_string(base_name);
        let variant_normalized = self.normalize_string(variant);
        
        self.name_variants
            .entry(base_normalized.clone())
            .or_default()
            .push(variant_normalized.clone());
        
        self.name_variants
            .entry(variant_normalized)
            .or_default()
            .push(base_normalized);
    }

    // Mantieni tutti gli altri metodi esistenti...
    pub fn parse_name(&self, input: &str) -> Result<ParsedName, RitmoErr> {
        let parsed: Name = Name::parse(input)
            .ok_or_else(|| NameManagerErrorInternal::NameParsingError(format!("Impossibile parsificare il nome: '{}'", input)))?;
        
        let given_name = parsed.given_name().unwrap_or("").to_string();
        let surname = parsed.surname().to_string();
        let middle_names: Vec<String> = parsed.middle_names()
            .map(|names| names.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let title = parsed.honorific_prefix().map(|s| s.to_string());
        let suffix = parsed.generational_suffix().map(|s| s.to_string());
        let display_name = parsed.display_first_last();
        
        Ok(ParsedName {
            given_name,
            surname,
            middle_names,
            title,
            suffix,
            display_name: display_name.to_string(),
        })
    }
    
    pub fn normalize_parsed_name_for_matching(&self, parsed_name: &ParsedName) -> String {
        let mut full_name_parts = Vec::new();
        
        if !parsed_name.given_name.is_empty() {
            full_name_parts.push(parsed_name.given_name.as_str());
        }
        
        for middle_name in &parsed_name.middle_names {
            if !middle_name.is_empty() {
                full_name_parts.push(middle_name.as_str());
            }
        }
        
        if !parsed_name.surname.is_empty() {
            full_name_parts.push(parsed_name.surname.as_str());
        }
        
        let combined_name = full_name_parts.join(" ");
        self.normalize_string(&combined_name)
    }

    // NUOVO: Metodi per il machine learning
    
    /// Avvia il processo di apprendimento automatico sui dati esistenti
    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        println!("Inizio training del modello ML per il riconoscimento delle varianti di nomi...");
        
        // 1. Raccogli tutti i nomi per l'analisi
        let all_names: Vec<String> = self.all_person_records
            .values()
            .map(|record| record.normalized_key.clone())
            .collect();
        
        // 2. Crea cluster di nomi simili
        self.ml_learner.create_name_clusters(&all_names, &self.double_metaphone)?;
        
        // 3. Identifica pattern nelle varianti
        self.ml_learner.identify_variant_patterns()?;
        
        // 4. Aggiorna le varianti apprese
        self.apply_learned_variants()?;
        
        println!("Training completato. {} pattern appresi, {} cluster creati.", 
                 self.ml_learner.learned_patterns.len(), 
                 self.ml_learner.name_clusters.len());
        
        Ok(())
    }
    
    /// Applica le varianti apprese al sistema di matching
    fn apply_learned_variants(&mut self) -> Result<(), RitmoErr> {
        let high_confidence_patterns: Vec<_> = self.ml_learner.learned_patterns
            .iter()
            .filter(|pattern| pattern.confidence >= self.ml_learner.minimum_confidence)
            .cloned()
            .collect();
        
        for pattern in high_confidence_patterns {
            self.add_name_variant(&pattern.base_form, &pattern.variant_form);
        }
        
        Ok(())
    }
    
    /// Incrementa il training con nuovi dati osservati
    pub fn incremental_learning(&mut self, observed_matches: Vec<(String, String, f64)>) -> Result<(), RitmoErr> {
        for (name1, name2, confidence) in observed_matches {
            if confidence >= 0.8 {
                self.ml_learner.add_observed_variant(&name1, &name2, confidence)?;
            }
        }
        
        // Ritraina periodicamente
        if self.ml_learner.pattern_frequency.len() % 100 == 0 {
            self.train_ml_model()?;
        }
        
        Ok(())
    }

    pub async fn load_names_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, given_name, surname, middle_names, title, suffix, display_name, normalized_key, confidence
            FROM people
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nel recupero dei nomi dal DB: {}", e)))?;
        for row in rows {
            let id: i64 = row.try_get("id")?;
            let original_input: String = row.try_get("name")?;
            let given_name: String = row.try_get("given_name")?;
            let surname: String = row.try_get("surname")?;
            let middle_names_str: Option<String> = row.try_get("middle_names")?;
            let middle_names: Vec<String> = middle_names_str
                .map(|s| s.split(',').filter(|part| !part.trim().is_empty()).map(|part| part.trim().to_string()).collect())
                .unwrap_or_default();
            let title: Option<String> = row.try_get("title")?;
            let suffix: Option<String> = row.try_get("suffix")?;
            let display_name: String = row.try_get("display_name")?;
            let normalized_key: String = row.try_get("normalized_key")?;
            let confidence: f64 = row.try_get("confidence")?;
            let aliases: Vec<String> = Vec::new(); // Per ora, gestiamo gli alias come un campo vuoto se non li recuperiamo
            let parsed_name = ParsedName {
                given_name,
                surname,
                middle_names,
                title,
                suffix,
                display_name,
            };
            let person_record = PersonRecord {
                id,
                original_input,
                parsed_name,
                normalized_key,
// ATTENZIONE !!!!
// qui occorre modificare il DB per memorizzare anche questa chiave
                phonetic_key: "".to_string(),
                confidence,
                verified: true,
                aliases,
            };
            self.add_person_record(person_record)?;
        }
        Ok(())
    }
    async fn save_single_person_record_in_tx(
        &self,
        transaction: &mut Transaction<'_, Sqlite>, // Accetta una transazione mutabile
        record: &PersonRecord,
    ) -> Result<(), RitmoErr> {
        let middle_names_str = if record.parsed_name.middle_names.is_empty() {
            None
        } else {
            Some(record.parsed_name.middle_names.join(", "))
        };
        let result = query(
            r#"
            INSERT OR REPLACE INTO people (
                id, name, given_name, surname, middle_names, title, suffix,
                display_name, normalized_key, confidence, verified, created_at, updated_at, source
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'), ?)
            "#,
        )
        .bind(record.id)
        .bind(&record.original_input)
        .bind(&record.parsed_name.given_name)
        .bind(&record.parsed_name.surname)
        .bind(middle_names_str)
        .bind(&record.parsed_name.title)
        .bind(&record.parsed_name.suffix)
        .bind(&record.parsed_name.display_name)
        .bind(&record.normalized_key)
        .bind(record.confidence)
        .bind(record.verified)
        .bind("biblioteca") // Rendi questo configurabile se necessario
        .execute(&mut **transaction) // Esegui la query all'interno della transazione
        .await
        .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel salvare il record persona nel DB durante la transazione: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(RitmoErr::DatabaseTransactionError(format!("Nessuna riga modificata per ID {} durante il salvataggio del record.", record.id)));
        }
        Ok(())
    }

    pub async fn save_person_records_to_db(
        &self,
        pool: &SqlitePool,
        records: &Vec<PersonRecord>, // O &Vec<PersonRecord> se non vuoi consumare il vettore
    ) -> Result<(), RitmoErr> {
        let mut transaction = pool.begin()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nell'avviare la transazione: {}", e)))?;
        for record in records {
            self.save_single_person_record_in_tx(&mut transaction, &record).await?;
        }
        transaction.commit()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel commettere la transazione: {}", e)))?;
        Ok(())
    }

    pub async fn process_and_add_new_names(&mut self, pool: &SqlitePool, names_to_process: Vec<String>) -> Result<(), RitmoErr> {
        let mut new_person_records_to_add: Vec<PersonRecord> = Vec::new();
        let mut current_max_id: i64 = self.all_person_records.keys().max().copied().unwrap_or(0); // Trova l'ID massimo corrente
        println!("Inizio elaborazione di {} nomi per l'aggiunta al DB...", names_to_process.len());
        for name_input in names_to_process {
            let match_result = self.find_matches(&name_input);
            let should_add_new = match match_result {
                MatchResult::ExactMatch(_) => {
                    false
                },
                MatchResult::HighConfidenceMatch(matches) => {
                    if matches[0].confidence >= 0.95 { // Soglia per alta confidenza
                        false
                    } else {
                        true // Considera l'aggiunta se la confidenza è buona ma non altissima
                    }
                },
                MatchResult::PossibleMatches(_) | MatchResult::NoMatch => {
                    true
                },
            };
            if should_add_new {
                current_max_id += 1; // Incrementa l'ID per il nuovo record
                match self.create_person_record(&name_input, current_max_id) {
                    Ok(new_record) => {
                        println!("  Aggiunto nuovo record in memoria per '{}' con ID: {}", new_record.parsed_name.display_name, new_record.id);
                        new_person_records_to_add.push(new_record);
                    },
                    Err(e) => {
                        eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
                    }
                }
            }
        }
        if !new_person_records_to_add.is_empty() {
            println!("Salvataggio di {} nuovi record nel database...", new_person_records_to_add.len());
            self.save_person_records_to_db(pool, &new_person_records_to_add).await?; // Clona per mantenere i record in memoria
            println!("Salvataggio completato nel database.");
            for record in new_person_records_to_add {
                self.add_person_record(record)?;
            }
        } else {
            println!("Nessun nuovo nome da aggiungere al database.");
        }
        Ok(())
    } 
    // Altri metodi esistenti da mantenere...
}
