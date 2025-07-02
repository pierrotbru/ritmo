use crate::publishers::publishers_feedback::save_feedback;
use crate::publishers::publishers_feedback::FeedbackType;
use crate::publishers::publishers_ml::MLPublisherLearner;
use crate::names::utils::NameUtils;
use crate::RitmoErr;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PublisherRecord {
    pub id: i64,
    pub original_input: String,
    pub normalized_key: String,
    pub aliases: Vec<String>,
}

pub struct PublisherManager {
    pub publisher_utils: NameUtils, // Per normalizzazione e funzioni ausiliarie
    pub all_publisher_records: HashMap<i64, PublisherRecord>,
    pub normalized_key_index: HashMap<String, HashSet<i64>>,
    pub ml_learner: MLPublisherLearner,
}

impl PublisherManager {
    pub fn new() -> Self {
        let publisher_utils = NameUtils::new(HashMap::new());
        Self {
            publisher_utils,
            all_publisher_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            ml_learner: MLPublisherLearner::new(),
        }
    }

    // Aggiunta di record nuovo
    pub fn add_new_record(&mut self, current_id: i64, publisher_input: &str) -> Result<(), RitmoErr> {
        let record = self.create_publisher_record(publisher_input, current_id)?;
        self.add_publisher_record(record)?;
        Ok(())
    }

    fn create_publisher_record(&self, publisher_input: &str, id: i64) -> Result<PublisherRecord, RitmoErr> {
        let normalized_key = self.publisher_utils.normalize_string(publisher_input);
        Ok(PublisherRecord {
            id,
            original_input: publisher_input.to_string(),
            normalized_key,
            aliases: vec![],
        })
    }

    pub fn add_publisher_record(&mut self, record: PublisherRecord) -> Result<(), RitmoErr> {
        let id = record.id;
        let normalized_key = record.normalized_key.clone();
        self.all_publisher_records.insert(id, record.clone());
        self.normalized_key_index.entry(normalized_key)
            .or_default()
            .insert(id);
        for alias in &record.aliases {
            let normalized_alias = self.publisher_utils.normalize_string(alias);
            self.normalized_key_index.entry(normalized_alias)
                .or_default()
                .insert(id);
        }
        Ok(())
    }

    pub fn add_alias_to_publisher_record(&mut self, publisher_id: i64, alias_name: String) -> Result<(), RitmoErr> {
        if let Some(record) = self.all_publisher_records.get_mut(&publisher_id) {
            if !record.aliases.contains(&alias_name) && record.original_input != alias_name {
                 record.aliases.push(alias_name.clone());
                 let normalized_alias = self.publisher_utils.normalize_string(&alias_name);
                 self.normalized_key_index.entry(normalized_alias)
                     .or_default()
                     .insert(publisher_id);
            }
            Ok(())
        } else {
            Err(RitmoErr::DatabaseQueryFailed(format!("PublisherRecord con ID {} non trovato per aggiungere alias.", publisher_id)))
        }
    }

    // Clustering e training ML
    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        let all_publishers: Vec<String> = self.all_publisher_records.values()
            .map(|rec| rec.normalized_key.clone())
            .collect();
        self.ml_learner.create_publisher_clusters(&all_publishers);
        self.ml_learner.identify_variant_patterns();
        Ok(())
    }

    // Salvataggio coordinato (ML + publisher_utils)
    pub async fn save_ml_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        self.ml_learner.save_to_db(pool).await?;
        self.publisher_utils.save_to_db(pool).await?;
        Ok(())
    }

    pub async fn register_false_positive(
        &mut self,
        pool: &SqlitePool,
        publisher1: &str,
        publisher2: &str
    ) -> Result<(), RitmoErr> {
        save_feedback(pool, FeedbackType::FalsePositive, publisher1, publisher2).await
    }

    pub async fn register_false_negative(
        &mut self,
        pool: &SqlitePool,
        publisher1: &str,
        expected_variant: &str
    ) -> Result<(), RitmoErr> {
        save_feedback(pool, FeedbackType::FalseNegative, publisher1, expected_variant).await
    }
    // Caricamento da DB (da implementare se necessario)
    // pub async fn load_ml_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> { ... }
}
