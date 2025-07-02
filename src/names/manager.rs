use crate::ml::entity_learner::{MLEntityLearner, VariantPatternType};
use crate::ml::entity_persistence::{save_ml_to_db, load_ml_from_db, save_scalar_to_db, load_scalar_from_db};
use crate::names::models::PersonRecord;
use crate::names::utils::NameUtils;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use crate::RitmoErr;

const TRAINING_THRESHOLDS: &[usize] = &[100, 200, 500, 1000, 2000, 5000];
const TRAINING_REPEAT: usize = 5000;

pub struct NameManager {
    pub name_utils: NameUtils,
    pub all_person_records: HashMap<i64, PersonRecord>,
    pub normalized_key_index: HashMap<String, HashSet<i64>>,
    pub ml_learner: MLEntityLearner,
    last_trained_count: usize,
}

impl NameManager {
    pub fn new() -> Self {
        let name_utils = NameUtils::new(HashMap::new());
        Self {
            name_utils,
            all_person_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            ml_learner: MLEntityLearner::new(),
            last_trained_count: 0,
        }
    }

    pub fn add_new_record(&mut self, id: i64, name: &str) -> Result<(), RitmoErr> {
        // Esempio minimale di aggiunta record
        let parsed = self.name_utils.parse_name(name)?;
        let normalized_key = self.name_utils.normalize_parsed_name_for_matching(&parsed);
        let record = PersonRecord {
            id,
            original_input: name.to_string(),
            normalized_key: normalized_key.clone(),
            aliases: Vec::new(),
            ..Default::default()
        };
        self.normalized_key_index
            .entry(normalized_key)
            .or_insert_with(HashSet::new)
            .insert(id);
        self.all_person_records.insert(id, record);

        let current_count = self.all_person_records.len();
        if Self::should_train_ml(current_count, self.last_trained_count) {
            self.train_ml_model()?;
            self.last_trained_count = current_count;
        }
        Ok(())
    }

    fn should_train_ml(current_count: usize, last_trained_count: usize) -> bool {
        for &threshold in TRAINING_THRESHOLDS {
            if last_trained_count < threshold && current_count >= threshold {
                return true;
            }
        }
        if current_count >= TRAINING_REPEAT
            && (current_count / TRAINING_REPEAT) > (last_trained_count / TRAINING_REPEAT)
        {
            return true;
        }
        false
    }

    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        let all_names: Vec<String> = self.all_person_records.values()
            .map(|r| r.normalized_key.clone())
            .collect();
        self.ml_learner.create_clusters(&all_names);
        self.ml_learner.identify_variant_patterns(
            &Self::classify_name_pattern,
            &Self::calc_pattern_confidence,
        );
        Ok(())
    }

    fn classify_name_pattern(a: &str, b: &str, edit_dist: usize) -> VariantPatternType {
        VariantPatternType::Typo // Semplificato, sostituisci con la logica completa
    }

    fn calc_pattern_confidence(_a: &str, _b: &str, _pattern_type: &VariantPatternType, sim: f64) -> f64 {
        sim
    }

    pub async fn save_ml_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let mut tx = pool.begin().await?;
        save_ml_to_db(&mut tx, &self.ml_learner, "person").await?;
        save_scalar_to_db(&mut tx, "person_last_trained_count", &self.last_trained_count).await?;
        self.name_utils.save_to_db(pool).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn load_ml_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        self.ml_learner = load_ml_from_db(pool, "person").await?;
        self.last_trained_count = load_scalar_from_db(pool, "person_last_trained_count")
            .await?.unwrap_or(0);
        self.name_utils = NameUtils::load_from_db(pool).await?;
        Ok(())
    }
}
