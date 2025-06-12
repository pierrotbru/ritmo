use serde::{Serialize, Deserialize};
use crate::db::enhanced_name_manager_2::{NameVariantPattern, NameCluster, VariantPatternType};
use crate::errors::RitmoErr;
use strsim::{jaro_winkler, levenshtein};
use std::collections::HashMap;
use rphonetic::{DoubleMetaphone, Encoder};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLNameLearner {
    pub learned_patterns: Vec<NameVariantPattern>,
    pub name_clusters: Vec<NameCluster>,
    pub pattern_frequency: HashMap<String, usize>,
    pub minimum_confidence: f64,
    pub minimum_frequency: usize,
}

// NUOVO: Implementazione del sistema ML
impl MLNameLearner {
    pub fn new() -> Self {
        Self {
            learned_patterns: Vec::new(),
            name_clusters: Vec::new(),
            pattern_frequency: HashMap::new(),
            minimum_confidence: 0.85,
            minimum_frequency: 3,
        }
    }
    
    /// Crea cluster di nomi basati su similarità fonetica e testuale
    pub fn create_name_clusters(&mut self, names: &[String], encoder: &DoubleMetaphone) -> Result<(), RitmoErr> {
        let mut phonetic_groups: HashMap<String, Vec<String>> = HashMap::new();
        
        // Raggruppa per similarità fonetica
        for name in names {
            phonetic_groups.entry(encoder.encode(name)).or_default().push(name.clone());
        }
        
        // Crea cluster da gruppi con almeno 2 membri
        let mut cluster_id = 0;
        for (phonetic_sig, group_names) in phonetic_groups {
            if group_names.len() >= 2 {
                // Trova il centroide (nome più comune o più rappresentativo)
                let centroid = self.find_centroid(&group_names);
                
                let cluster = NameCluster {
                    cluster_id,
                    members: group_names.clone(),
                    centroid,
                    phonetic_signature: phonetic_sig,
                    confidence: self.calculate_cluster_confidence(&group_names),
                };
                
                self.name_clusters.push(cluster);
                cluster_id += 1;
            }
        }
        
        Ok(())
    }
    
    /// Identifica pattern comuni nelle varianti di nomi
    pub fn identify_variant_patterns(&mut self) -> Result<(), RitmoErr> {
        // Analizza ogni cluster per identificare pattern
        for cluster in &self.name_clusters {
            for i in 0..cluster.members.len() {
                for j in (i + 1)..cluster.members.len() {
                    let name1 = &cluster.members[i];
                    let name2 = &cluster.members[j];
                    
                    if let Some(pattern) = self.analyze_name_pair(name1, name2) {
                        // Incrementa la frequenza del pattern
                        let pattern_key = format!("{}→{}", pattern.pattern_type as u8, 
                                                 self.extract_pattern_signature(&pattern));
                        *self.pattern_frequency.entry(pattern_key).or_insert(0) += 1;
                        
                        // Aggiungi solo se supera la soglia
                        if pattern.confidence >= self.minimum_confidence && 
                           self.pattern_frequency.get(&format!("{}→{}", pattern.pattern_type as u8, 
                                                              self.extract_pattern_signature(&pattern)))
                               .unwrap_or(&0) >= &self.minimum_frequency {
                            self.learned_patterns.push(pattern);
                        }
                    }
                }
            }
        }
        
        // Rimuovi pattern duplicati e ordina per confidenza
        self.learned_patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        self.learned_patterns.dedup_by(|a, b| {
            a.base_form == b.base_form && a.variant_form == b.variant_form
        });
        
        Ok(())
    }
    
    /// Analizza una coppia di nomi per identificare il tipo di variante
    fn analyze_name_pair(&self, name1: &str, name2: &str) -> Option<NameVariantPattern> {
        let edit_dist = levenshtein(name1, name2);
        let phonetic_sim = jaro_winkler(name1, name2);
        
        // Soglia minima per considerare una variante
        if phonetic_sim < 0.7 {
            return None;
        }
        
        let pattern_type = self.classify_variant_type(name1, name2, edit_dist);
        let confidence = self.calculate_pattern_confidence(name1, name2, &pattern_type, phonetic_sim);
        
        Some(NameVariantPattern {
            base_form: name1.to_string(),
            variant_form: name2.to_string(),
            pattern_type,
            confidence,
            frequency: 1,
            phonetic_similarity: phonetic_sim,
            edit_distance: edit_dist,
        })
    }
    
    /// Classifica il tipo di variante tra due nomi
    fn classify_variant_type(&self, name1: &str, name2: &str, edit_distance: usize) -> VariantPatternType {
        let len1 = name1.len();
        let len2 = name2.len();
        
        // Analizza pattern di trasformazione
        if edit_distance <= 2 && (len1 as i32 - len2 as i32).abs() <= 2 {
            // Probabilmente un typo o variante fonetica
            if self.has_transliteration_pattern(name1, name2) {
                VariantPatternType::Transliteration
            } else {
                VariantPatternType::Phonetic
            }
        } else if len1 > len2 && name1.starts_with(name2) {
            VariantPatternType::Suffix
        } else if len2 > len1 && name2.starts_with(name1) {
            VariantPatternType::Prefix
        } else if (len1 as i32 - len2 as i32).abs() > 3 {
            VariantPatternType::Abbreviation
        } else if name1.contains('-') || name2.contains('-') {
            VariantPatternType::Compound
        } else {
            VariantPatternType::Phonetic
        }
    }
    
    /// Verifica se due nomi seguono pattern di traslitterazione
    fn has_transliteration_pattern(&self, name1: &str, name2: &str) -> bool {
        // Pattern comuni di traslitterazione
        let transliteration_pairs = [
            ("ch", "c"), ("ph", "f"), ("th", "t"),
            ("ov", "of"), ("ev", "ef"), ("ić", "ic"),
            ("č", "c"), ("š", "s"), ("ž", "z"),
        ];
        
        for (pattern1, pattern2) in &transliteration_pairs {
            if (name1.contains(pattern1) && name2.contains(pattern2)) ||
               (name1.contains(pattern2) && name2.contains(pattern1)) {
                return true;
            }
        }
        false
    }
    
    /// Calcola la confidenza di un pattern identificato
    fn calculate_pattern_confidence(&self, name1: &str, name2: &str, pattern_type: &VariantPatternType, phonetic_sim: f64) -> f64 {
        let mut confidence = phonetic_sim;
        
        // Bonus per pattern ben definiti
        match pattern_type {
            VariantPatternType::Transliteration => confidence *= 1.1,
            VariantPatternType::Suffix | VariantPatternType::Prefix => confidence *= 1.05,
            VariantPatternType::Abbreviation => confidence *= 0.95,
            _ => {}
        }
        
        // Penalità per nomi molto diversi in lunghezza
        let len_diff = (name1.len() as i32 - name2.len() as i32).abs() as f64;
        if len_diff > 3.0 {
            confidence *= 0.9;
        }
        
        confidence.min(1.0)
    }
    
    /// Trova il centroide di un cluster (nome più rappresentativo)
    fn find_centroid(&self, names: &[String]) -> String {
        if names.is_empty() {
            return String::new();
        }
        
        // Semplice euristica: il nome con lunghezza mediana
        let mut sorted_names = names.to_vec();
        sorted_names.sort_by_key(|name| name.len());
        sorted_names[sorted_names.len() / 2].clone()
    }
    
    /// Calcola la confidenza di un cluster
    fn calculate_cluster_confidence(&self, names: &[String]) -> f64 {
        if names.len() < 2 {
            return 0.0;
        }
        
        let mut total_similarity = 0.0;
        let mut comparisons = 0;
        
        for i in 0..names.len() {
            for j in (i + 1)..names.len() {
                total_similarity += jaro_winkler(&names[i], &names[j]);
                comparisons += 1;
            }
        }
        
        if comparisons > 0 {
            total_similarity / comparisons as f64
        } else {
            0.0
        }
    }
    
    /// Estrae una firma unica per un pattern
    fn extract_pattern_signature(&self, pattern: &NameVariantPattern) -> String {
        format!("{}→{}", 
                pattern.base_form.chars().take(3).collect::<String>(),
                pattern.variant_form.chars().take(3).collect::<String>())
    }
    
    /// Cerca una variante appresa per due nomi
    pub fn find_learned_variant(&self, name1: &str, name2: &str) -> Option<&NameVariantPattern> {
        self.learned_patterns.iter().find(|pattern| {
            (pattern.base_form == name1 && pattern.variant_form == name2) ||
            (pattern.base_form == name2 && pattern.variant_form == name1)
        })
    }

#[allow(dead_code)]    
    /// Aggiunge una variante osservata per l'apprendimento incrementale
    pub fn add_observed_variant(&mut self, name1: &str, name2: &str, confidence: f64,) -> Result<(), RitmoErr> {
        if let Some(pattern) = self.analyze_name_pair(name1, name2) {
            // Cerca se esiste già
            if let Some(existing) = self.learned_patterns.iter_mut().find(|p| {
                (p.base_form == name1 && p.variant_form == name2) ||
                (p.base_form == name2 && p.variant_form == name1)
            }) {
                // Aggiorna frequenza e confidenza
                existing.frequency += 1;
                existing.confidence = (existing.confidence + confidence) / 2.0;
            } else {
                // Aggiungi nuovo pattern
                self.learned_patterns.push(pattern);
            }
        }
        
        Ok(())
    }
}