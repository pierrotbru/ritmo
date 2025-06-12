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
    pub phonetic_key: String, 
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
    NameOrder,
//    Phonetic,
//    Abbreviated,
    Typo,
    Alias,
    PhoneticSimilar,
    TypoMinor,
    TypoMajor,
    Learned,
}

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
    Suffix,        
    Prefix,        
    Phonetic,      
    Transliteration, 
    Abbreviation,  
    Compound,      
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameCluster {
    pub cluster_id: usize,
    pub members: Vec<String>,
    pub centroid: String,
    pub phonetic_signature: String,
    pub confidence: f64,
}
#[allow(dead_code)]
pub struct NameManager {
    fuzzy_matcher: SkimMatcherV2,
    double_metaphone: DoubleMetaphone,
    common_abbreviations: HashMap<String, Vec<String>>,
    similarity_threshold: f64,
    typo_threshold: f64,
    pub all_person_records: HashMap<i64, PersonRecord>,
    normalized_key_index: HashMap<String, HashSet<i64>>,
    phonetic_key_index: HashMap<String, HashSet<i64>>,
    name_variants: HashMap<String, Vec<String>>,
    ml_learner: MLNameLearner,
}

impl NameManager {
    pub fn new() -> Self {
        let mut common_abbreviations = HashMap::new();
        common_abbreviations.insert("giuseppe".to_string(), vec!["peppe".to_string(), "beppe".to_string()]);
        common_abbreviations.insert("giovanni".to_string(), vec!["gianni".to_string(), "gian".to_string()]);
        common_abbreviations.insert("francesco".to_string(), vec!["franco".to_string(), "checco".to_string()]);

        let mut name_variants = HashMap::new();
        name_variants.insert("anton".to_string(), vec!["antonio".to_string(), "antony".to_string()]);
        name_variants.insert("pavlovic".to_string(), vec!["pavlociv".to_string(), "pavlovič".to_string()]);
        name_variants.insert("cechov".to_string(), vec!["chekhov".to_string(), "čechov".to_string(), "tchekhov".to_string()]);
        name_variants.insert("franc".to_string(), vec!["frank".to_string(), "franck".to_string(), "francesco".to_string()]);
        
        Self {
            fuzzy_matcher: SkimMatcherV2::default(),
            double_metaphone: DoubleMetaphone::default(), 
            common_abbreviations,
            similarity_threshold: 0.75, 
            typo_threshold: 0.85, 
            all_person_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            phonetic_key_index: HashMap::new(), 
            name_variants, 
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

    fn normalize_string(&self, text: &str) -> String {
        let normalized = text
            .nfc()
            .collect::<String>()
            .to_lowercase()
            
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
                
                c if c.is_alphabetic() || c.is_whitespace() => c,
                _ => ' ',
            })
            .collect::<String>();

        normalized.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    fn generate_phonetic_key(&self, text: &str) -> String {
        let normalized = self.normalize_string(text);
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        let mut phonetic_parts = Vec::new();
        
        for part in parts {

            phonetic_parts.push(self.double_metaphone.encode(part));
        }
        
        phonetic_parts.join(" ")
    }

    fn normalized_levenshtein_distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            return 1.0;
        }
        1.0 - (levenshtein(s1, s2) as f64 / max_len)
    }

    fn are_known_variants(&self, name1: &str, name2: &str) -> bool {
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

    fn add_person_record(&mut self, record: PersonRecord) -> Result<(), RitmoErr> {
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

    pub fn find_matches(&self, input_name: &str) -> MatchResult {
        let parsed_input_res = self.parse_name(input_name);
        if parsed_input_res.is_err() {
            return MatchResult::NoMatch;
        }
        
        let parsed_input = parsed_input_res.unwrap();
        let normalized_input = self.normalize_parsed_name_for_matching(&parsed_input);
        let phonetic_input = self.generate_phonetic_key(&normalized_input);
        
        let mut candidate_ids: HashSet<i64> = HashSet::new();

        if let Some(ids) = self.normalized_key_index.get(&normalized_input) {
            candidate_ids.extend(ids);
        }

        if let Some(ids) = self.phonetic_key_index.get(&phonetic_input) {
            candidate_ids.extend(ids);
        }

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

                let direct_score = jaro_winkler(&normalized_input, &person.normalized_key);
                if direct_score > best_score {
                    best_score = direct_score;
                    let match_type = if direct_score >= 0.99 {
                        MatchType::Exact
                    } else if direct_score >= self.typo_threshold {
                        
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

                if best_score < 1.0 {
                    if let Some(learned_variant) = self.ml_learner.find_learned_variant(&normalized_input, &person.normalized_key) {
                        best_score = learned_variant.confidence.max(0.88); 
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: learned_variant.confidence,
                            match_type: MatchType::Learned,
                            confidence: learned_variant.confidence * person.confidence,
                        });
                    }
                }

                if best_score < 1.0 {
                    if self.are_known_variants(&normalized_input, &person.normalized_key) {
                        best_score = 0.95; 
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: 0.95,
                            match_type: MatchType::Alias,
                            confidence: 0.95 * person.confidence,
                        });
                    }
                }

                if best_score < 1.0 {
                    let phonetic_score = jaro_winkler(&phonetic_input, &person.phonetic_key);
                    if phonetic_score >= 0.8 && phonetic_score > best_score * 0.9 {
                        best_score = phonetic_score * 0.9; 
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: phonetic_score,
                            match_type: MatchType::PhoneticSimilar,
                            confidence: phonetic_score * person.confidence * 0.85,
                        });
                    }
                }

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

        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

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

    fn create_person_record(&self, input: &str, id: i64) -> Result<PersonRecord, RitmoErr> {
        let parsed_name = self.parse_name(input)?;
        let normalized_key = self.normalize_parsed_name_for_matching(&parsed_name);
        let phonetic_key = self.generate_phonetic_key(&normalized_key);
        
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

    fn add_name_variant(&mut self, base_name: &str, variant: &str) {
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

    fn parse_name(&self, input: &str) -> Result<ParsedName, RitmoErr> {
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
    
    fn normalize_parsed_name_for_matching(&self, parsed_name: &ParsedName) -> String {
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

    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        println!("Inizio training del modello ML per il riconoscimento delle varianti di nomi...");

        let all_names: Vec<String> = self.all_person_records
            .values()
            .map(|record| record.normalized_key.clone())
            .collect();

        self.ml_learner.create_name_clusters(&all_names, &self.double_metaphone)?;

        self.ml_learner.identify_variant_patterns()?;

        self.apply_learned_variants()?;
        
        println!("Training completato. {} pattern appresi, {} cluster creati.", 
                 self.ml_learner.learned_patterns.len(), 
                 self.ml_learner.name_clusters.len());
        
        Ok(())
    }

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

#[allow(dead_code)]
    fn incremental_learning(&mut self, observed_matches: Vec<(String, String, f64)>) -> Result<(), RitmoErr> {
        for (name1, name2, confidence) in observed_matches {
            if confidence >= 0.8 {
                self.ml_learner.add_observed_variant(&name1, &name2, confidence)?;
            }
        }

        if self.ml_learner.pattern_frequency.len() % 100 == 0 {
            self.train_ml_model()?;
        }
        
        Ok(())
    }

#[allow(dead_code)]
    pub async fn load_names_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, given_name, surname, middle_names, title, suffix, display_name, normalized_key, phonetic_key, confidence
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
            let phonetic_key: String = row.try_get("phonetic_key")?;
            let confidence: f64 = row.try_get("confidence")?;
            let aliases: Vec<String> = Vec::new(); 
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
                phonetic_key,
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
        transaction: &mut Transaction<'_, Sqlite>, 
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
                display_name, normalized_key, phonetic_key, confidence, verified, created_at, updated_at, source
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'), ?)
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
        .bind(&record.phonetic_key)
        .bind(record.confidence)
        .bind(record.verified)
        .bind("biblioteca") 
        .execute(&mut **transaction) 
        .await
        .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel salvare il record persona nel DB durante la transazione: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(RitmoErr::DatabaseTransactionError(format!("Nessuna riga modificata per ID {} durante il salvataggio del record.", record.id)));
        }
        Ok(())
    }

#[allow(dead_code)]
    /// salva nel DB i records di persone contenuti nel vettore records
    pub async fn save_person_records_to_db(
        &mut self,
        pool: &SqlitePool,
        records: &Vec<PersonRecord>, 
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

        self.train_ml_model()?; // Esegue il training del modello ML.
        self.save_ml_data_to_db(pool).await?;

        Ok(())
    }

    /// Salva tutti i PersonRecord presenti nella HashMap `all_person_records` nel database, utilizzando una singola transazione.
    pub async fn save_manager_person_records_to_db(
        &self, // `&self` è necessario per accedere a `self.all_person_records`
        pool: &SqlitePool,
    ) -> Result<(), RitmoErr> {
        // Inizia una nuova transazione. Mappa gli errori con il nuovo tipo.
        let mut transaction = pool.begin()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nell'avviare la transazione: {}", e)))?;

        for record in self.all_person_records.values() {
            self.save_single_person_record_in_tx(&mut transaction, record).await?;
        }
        transaction.commit()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel commettere la transazione: {}", e)))?;

        self.save_ml_data_to_db(pool).await?;

        Ok(())
    }

#[allow(dead_code)]
    /// aggiunge al database i nomi contenuti nel vettore in ingresso, ed inoltre li aggiunge anche a self
    /// ritorna un vettore contentente gli IDs dei nomi inseriti
    pub async fn process_and_add_new_names(&mut self, pool: &SqlitePool, names_to_process: Vec<String>) -> Result<Vec<i64>, RitmoErr> {
        let mut new_person_records_to_add: Vec<PersonRecord> = Vec::new();
        let mut new_person_id: Vec<i64> = Vec::new();
        let mut current_max_id: i64 = self.all_person_records.keys().max().copied().unwrap_or(0); 
        for name_input in names_to_process {
            let match_result = self.find_matches(&name_input);
            let should_add_new = match match_result {
                MatchResult::ExactMatch(_) => {
                    false
                },
                MatchResult::HighConfidenceMatch(matches) => {
                    if matches[0].confidence >= 0.95 { 
                        false
                    } else {
                        true 
                    }
                },
                MatchResult::PossibleMatches(_) | MatchResult::NoMatch => {
                    true
                },
            };
            if should_add_new {
                current_max_id += 1; 
                match self.create_person_record(&name_input, current_max_id) {
                    Ok(new_record) => {
                        new_person_records_to_add.push(new_record);
                        new_person_id.push(current_max_id);
                    },
                    Err(e) => {
                        eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
                    }
                }
            }
        }
        if !new_person_records_to_add.is_empty() {
            for record in &new_person_records_to_add {
                self.add_person_record(record.clone())?;
            }
            self.save_person_records_to_db(pool, &new_person_records_to_add).await?; 
        }
        return Ok(new_person_id);
    } 

    async fn save_ml_data_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let mut tx = pool.begin().await
            .map_err(|e| RitmoErr::MLError(format!("Failed to start transaction: {}", e)))?;

        let patterns_json = serde_json::to_string(&self.ml_learner.learned_patterns)
            .map_err(|e| RitmoErr::MLError(format!("Failed to serialize learned_patterns: {}", e)))?;
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json) 
                     VALUES ((SELECT id FROM ml_name_data WHERE data_type = 'learned_patterns'), 'learned_patterns', ?)")
            .bind(&patterns_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to save learned_patterns: {}", e)))?;

        let clusters_json = serde_json::to_string(&self.ml_learner.name_clusters)
            .map_err(|e| RitmoErr::MLError(format!("Failed to serialize name_clusters: {}", e)))?;
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json) 
                     VALUES ((SELECT id FROM ml_name_data WHERE data_type = 'name_clusters'), 'name_clusters', ?)")
            .bind(&clusters_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to save name_clusters: {}", e)))?;

        let frequency_json = serde_json::to_string(&self.ml_learner.pattern_frequency)
            .map_err(|e| RitmoErr::MLError(format!("Failed to serialize pattern_frequency: {}", e)))?;
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json) 
                     VALUES ((SELECT id FROM ml_name_data WHERE data_type = 'pattern_frequency'), 'pattern_frequency', ?)")
            .bind(&frequency_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to save pattern_frequency: {}", e)))?;

        let variants_json = serde_json::to_string(&self.name_variants)
            .map_err(|e| RitmoErr::MLError(format!("Failed to serialize name_variants: {}", e)))?;
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json) 
                     VALUES ((SELECT id FROM ml_name_data WHERE data_type = 'name_variants'), 'name_variants', ?)")
            .bind(&variants_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to save name_variants: {}", e)))?;

        let config = serde_json::json!({
            "minimum_confidence": self.ml_learner.minimum_confidence,
            "minimum_frequency": self.ml_learner.minimum_frequency
        });
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json) 
                     VALUES ((SELECT id FROM ml_name_data WHERE data_type = 'ml_config'), 'ml_config', ?)")
            .bind(config.to_string())
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to save ml_config: {}", e)))?;
        
        tx.commit().await
            .map_err(|e| RitmoErr::MLError(format!("Failed to commit ML data transaction: {}", e)))?;
        
        println!("Dati ML salvati nel database: {} pattern, {} cluster, {} frequenze", 
                 self.ml_learner.learned_patterns.len(),
                 self.ml_learner.name_clusters.len(),
                 self.ml_learner.pattern_frequency.len());
        Ok(())
    }
    
#[allow(dead_code)]
    pub async fn load_ml_data_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        
        if let Some(row) = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = 'learned_patterns' ORDER BY updated_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to load learned_patterns: {}", e)))?
        {
            let patterns_json: String = row.get("data_json");
            self.ml_learner.learned_patterns = serde_json::from_str(&patterns_json)
                .map_err(|e| RitmoErr::MLError(format!("Failed to deserialize learned_patterns: {}", e)))?;
        }

        if let Some(row) = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = 'name_clusters' ORDER BY updated_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to load name_clusters: {}", e)))?
        {
            let clusters_json: String = row.get("data_json");
            self.ml_learner.name_clusters = serde_json::from_str(&clusters_json)
                .map_err(|e| RitmoErr::MLError(format!("Failed to deserialize name_clusters: {}", e)))?;
        }

        if let Some(row) = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = 'pattern_frequency' ORDER BY updated_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to load pattern_frequency: {}", e)))?
        {
            let frequency_json: String = row.get("data_json");
            self.ml_learner.pattern_frequency = serde_json::from_str(&frequency_json)
                .map_err(|e| RitmoErr::MLError(format!("Failed to deserialize pattern_frequency: {}", e)))?;
        }

        if let Some(row) = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = 'name_variants' ORDER BY updated_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to load name_variants: {}", e)))?
        {
            let variants_json: String = row.get("data_json");
            self.name_variants = serde_json::from_str(&variants_json)
                .map_err(|e| RitmoErr::MLError(format!("Failed to deserialize name_variants: {}", e)))?;
        }

        if let Some(row) = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = 'ml_config' ORDER BY updated_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| RitmoErr::MLError(format!("Failed to load ml_config: {}", e)))?
        {
            let config_json: String = row.get("data_json");
            let config: serde_json::Value = serde_json::from_str(&config_json)
                .map_err(|e| RitmoErr::MLError(format!("Failed to deserialize ml_config: {}", e)))?;
            
            if let Some(confidence) = config.get("minimum_confidence").and_then(|v| v.as_f64()) {
                self.ml_learner.minimum_confidence = confidence;
            }
            if let Some(frequency) = config.get("minimum_frequency").and_then(|v| v.as_u64()) {
                self.ml_learner.minimum_frequency = frequency as usize;
            }
        }
        
        println!("Dati ML caricati dal database: {} pattern, {} cluster, {} frequenze, {} varianti", 
                 self.ml_learner.learned_patterns.len(),
                 self.ml_learner.name_clusters.len(),
                 self.ml_learner.pattern_frequency.len(),
                 self.name_variants.len());
        Ok(())
    }
}
