use crate::names::utils::NameUtils;
use crate::ml::entity_learner::{MLEntityLearner, VariantPatternType};
use crate::ml::entity_persistence::{save_ml_to_db, load_ml_from_db};
//use crate::names::utils::NameUtils;
use std::collections::{HashMap, HashSet};
use sqlx::SqlitePool;
use crate::RitmoErr;

#[derive(Debug, Clone)]
pub struct PublisherRecord {
    pub id: i64,
    pub original_input: String,
    pub normalized_key: String,
    pub aliases: Vec<String>,
}

pub struct PublisherManager {
    pub publisher_utils: NameUtils,
    pub all_publisher_records: HashMap<i64, PublisherRecord>,
    pub normalized_key_index: HashMap<String, HashSet<i64>>,
    pub ml_learner: MLEntityLearner,
}

impl PublisherManager {
    pub fn new() -> Self {
        let publisher_utils = NameUtils::new(HashMap::new());
        Self {
            publisher_utils,
            all_publisher_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            ml_learner: MLEntityLearner::new(),
        }
    }

    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        let all_publishers: Vec<String> = self.all_publisher_records.values()
            .map(|r| r.normalized_key.clone())
            .collect();
        self.ml_learner.create_clusters(&all_publishers);
        self.ml_learner.identify_variant_patterns(
            &Self::classify_publisher_pattern,
            &Self::calc_pattern_confidence,
        );
        Ok(())
    }

    fn classify_publisher_pattern(a: &str, b: &str, edit_dist: usize) -> VariantPatternType {
        // Logica base, puoi specializzare se vuoi pattern particolari per editori
        VariantPatternType::Typo // Semplificato, sostituisci se serve logica specifica
    }

    fn calc_pattern_confidence(a: &str, b: &str, pattern_type: &VariantPatternType, sim: f64) -> f64 {
        sim
    }

    pub async fn save_ml_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let mut tx = pool.begin().await?;
        save_ml_to_db(&mut tx, &self.ml_learner, "publisher").await?;
        self.publisher_utils.save_to_db(pool).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn load_ml_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        self.ml_learner = load_ml_from_db(pool, "publisher").await?;
        self.publisher_utils = NameUtils::load_from_db(pool).await?;
        Ok(())
    }
}