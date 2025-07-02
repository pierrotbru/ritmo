use sqlx::Row;
use sqlx::SqlitePool;
use crate::names::NameVariantPattern;
use crate::names::NameCluster;
use crate::names::VariantPatternType;
use serde::{Serialize, Deserialize};
use crate::errors::RitmoErr;
use strsim::{jaro_winkler, levenshtein};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLNameLearner {
    pub learned_patterns: Vec<NameVariantPattern>,
    pub name_clusters: Vec<NameCluster>,
    pub pattern_frequency: HashMap<String, usize>,
    pub minimum_confidence: f64,
    pub minimum_frequency: usize,
}

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

    /// Crea cluster di nomi basati solo su similarità testuale
    pub fn create_name_clusters(&mut self, names: &[String]) -> Result<(), RitmoErr> {
        let mut clusters: Vec<NameCluster> = Vec::new();
        let mut used = vec![false; names.len()];
        let threshold = 0.85; // Soglia di similarità

        for (i, name1) in names.iter().enumerate() {
            if used[i] { continue; }
            let mut group = vec![name1.clone()];
            used[i] = true;

            for (j, name2) in names.iter().enumerate().skip(i + 1) {
                if !used[j] && jaro_winkler(name1, name2) > threshold {
                    group.push(name2.clone());
                    used[j] = true;
                }
            }
            if group.len() > 1 {
                let centroid = self.find_centroid(&group);
                clusters.push(NameCluster {
                    cluster_id: clusters.len(),
                    members: group.clone(),
                    centroid,
                    confidence: self.calculate_cluster_confidence(&group),
                });
            }
        }
        self.name_clusters = clusters;
        Ok(())
    }

    /// Identifica pattern comuni nelle varianti di nomi
    pub fn identify_variant_patterns(&mut self) -> Result<(), RitmoErr> {
        for cluster in &self.name_clusters {
            for i in 0..cluster.members.len() {
                for j in (i + 1)..cluster.members.len() {
                    let name1 = &cluster.members[i];
                    let name2 = &cluster.members[j];

                    if let Some(pattern) = self.analyze_name_pair(name1, name2) {
                        let pattern_key = format!("{}→{}", pattern.pattern_type as u8, self.extract_pattern_signature(&pattern));
                        *self.pattern_frequency.entry(pattern_key).or_insert(0) += 1;

                        if pattern.confidence >= self.minimum_confidence &&
                            self.pattern_frequency.get(&format!("{}→{}", pattern.pattern_type as u8, self.extract_pattern_signature(&pattern)))
                                .unwrap_or(&0) >= &self.minimum_frequency {
                            self.learned_patterns.push(pattern);
                        }
                    }
                }
            }
        }
        self.learned_patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        self.learned_patterns.dedup_by(|a, b| {
            a.base_form == b.base_form && a.variant_form == b.variant_form
        });

        Ok(())
    }

    /// Analizza una coppia di nomi per identificare il tipo di variante
    fn analyze_name_pair(&self, name1: &str, name2: &str) -> Option<NameVariantPattern> {
        let edit_dist = levenshtein(name1, name2);
        let sim = jaro_winkler(name1, name2);

        // Soglia minima per considerare una variante
        if sim < 0.7 {
            return None;
        }

        let pattern_type = self.classify_variant_type(name1, name2, edit_dist);
        let confidence = self.calculate_pattern_confidence(name1, name2, &pattern_type, sim);

        Some(NameVariantPattern {
            base_form: name1.to_string(),
            variant_form: name2.to_string(),
            pattern_type,
            confidence,
            frequency: 1,
            edit_distance: edit_dist,
        })
    }

    /// Classifica il tipo di variante tra due nomi
    fn classify_variant_type(&self, name1: &str, name2: &str, edit_distance: usize) -> VariantPatternType {
        let len1 = name1.len();
        let len2 = name2.len();

        if edit_distance <= 2 && (len1 as i32 - len2 as i32).abs() <= 2 {
            if self.has_transliteration_pattern(name1, name2) {
                VariantPatternType::Transliteration
            } else {
                VariantPatternType::Typo
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
            VariantPatternType::Other
        }
    }

    /// Verifica se due nomi seguono pattern di traslitterazione
    fn has_transliteration_pattern(&self, name1: &str, name2: &str) -> bool {
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
    fn calculate_pattern_confidence(&self, name1: &str, name2: &str, pattern_type: &VariantPatternType, sim: f64) -> f64 {
        let mut confidence = sim;

        match pattern_type {
            VariantPatternType::Transliteration => confidence *= 1.1,
            VariantPatternType::Suffix | VariantPatternType::Prefix => confidence *= 1.05,
            VariantPatternType::Abbreviation => confidence *= 0.95,
            _ => {}
        }

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
            if let Some(existing) = self.learned_patterns.iter_mut().find(|p| {
                (p.base_form == name1 && p.variant_form == name2) ||
                (p.base_form == name2 && p.variant_form == name1)
            }) {
                existing.frequency += 1;
                existing.confidence = (existing.confidence + confidence) / 2.0;
            } else {
                self.learned_patterns.push(pattern);
            }
        }
        Ok(())
    }

    // Le funzioni di salvataggio/caricamento su DB restano invariate

    async fn save_data<T: serde::Serialize>(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        data_type: &str,
        data_value: &T,
    ) -> Result<(), RitmoErr> {
        let json_string = serde_json::to_string(data_value)?;

        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json)
                      VALUES ((SELECT id FROM ml_name_data WHERE data_type = ?), ?, ?)")
            .bind(data_type)
            .bind(data_type)
            .bind(json_string)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn save_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let mut tx = pool.begin().await?;

        Self::save_data(&mut tx, "learned_patterns", &self.learned_patterns).await?;
        Self::save_data(&mut tx, "name_clusters", &self.name_clusters).await?;
        Self::save_data(&mut tx, "pattern_frequency", &self.pattern_frequency).await?;

        let config = serde_json::json!({
            "minimum_confidence": self.minimum_confidence,
            "minimum_frequency": self.minimum_frequency
        });
        Self::save_data(&mut tx, "ml_config", &config).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn load_data<T: for<'de> serde::Deserialize<'de>>(
        pool: &SqlitePool,
        data_type: &str,
    ) -> Result<Option<T>, RitmoErr> {
        let row = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = ?")
            .bind(data_type)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(r) => {
                let json_string: String = r.try_get("data_json")?;
                let data: T = serde_json::from_str(&json_string)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Carica un'istanza di MLNameLearner dal database.
    pub async fn load_from_db(pool: &SqlitePool) -> Result<Self, RitmoErr> {
        let learned_patterns = Self::load_data(pool, "learned_patterns")
            .await?
            .unwrap_or_else(Vec::new);

        let name_clusters = Self::load_data(pool, "name_clusters")
            .await?
            .unwrap_or_else(Vec::new);

        let pattern_frequency = Self::load_data(pool, "pattern_frequency")
            .await?
            .unwrap_or_else(HashMap::new);

        let config_json = Self::load_data::<serde_json::Value>(pool, "ml_config")
            .await?
            .unwrap_or_else(|| serde_json::json!({
                "minimum_confidence": 0.0,
                "minimum_frequency": 0
            }));

        let minimum_confidence = config_json["minimum_confidence"].as_f64()
            .unwrap_or(0.0);

        let minimum_frequency = config_json["minimum_frequency"].as_u64()
            .map(|u| u as usize)
            .unwrap_or(0);

        Ok(MLNameLearner {
            learned_patterns,
            name_clusters,
            pattern_frequency,
            minimum_confidence,
            minimum_frequency,
        })
    }

    /// Applica un feedback di falso positivo: riduce la confidenza o rimuove il pattern tra due nomi
    pub fn apply_false_positive(&mut self, name1: &str, name2: &str) {
        self.learned_patterns.retain(|pattern| {
            !(pattern.base_form == name1 && pattern.variant_form == name2)
                && !(pattern.base_form == name2 && pattern.variant_form == name1)
        });
        // Puoi anche abbassare solo la confidenza invece di rimuovere del tutto
    }

    /// Applica un feedback di falso negativo: aggiunge o rafforza il pattern tra due nomi
    pub fn apply_false_negative(&mut self, name1: &str, name2: &str) {
        // Usa già la logica incrementale esistente
        let _ = self.add_observed_variant(name1, name2, 1.0);
    }

}
