// src/name_matching/ml.rs

use crate::errors::RitmoErr;
use super::manager::NameManager;

impl NameManager { // Implementa metodi su NameManager
    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        println!("Inizio training del modello ML per il riconoscimento delle varianti di nomi...");

        let all_names: Vec<String> = self.all_person_records
            .values()
            .map(|record| record.normalized_key.clone())
            .collect();

        self.ml_learner.create_name_clusters(&all_names)?;
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
            // Usa il metodo di NameManager per aggiungere la variante,
            // che a sua volta userà NameUtils per la normalizzazione.
            self.add_internal_name_variant(&pattern.base_form, &pattern.variant_form);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn incremental_learning(&mut self, observed_matches: Vec<(String, String, f64)>) -> Result<(), RitmoErr> {
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
}
