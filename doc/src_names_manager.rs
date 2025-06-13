use crate::names::feedback::{save_feedback, FeedbackType};

impl NameManager {
    /// Esempio: ricezione feedback da interfaccia utente/CLI
    pub async fn register_false_positive(&mut self, pool: &SqlitePool, name1: &str, name2: &str) -> Result<(), RitmoErr> {
        save_feedback(pool, FeedbackType::FalsePositive, name1, name2).await?;
        self.ml_learner.apply_false_positive(name1, name2);
        Ok(())
    }

    pub async fn register_false_negative(&mut self, pool: &SqlitePool, name1: &str, name2: &str) -> Result<(), RitmoErr> {
        save_feedback(pool, FeedbackType::FalseNegative, name1, name2).await?;
        self.ml_learner.apply_false_negative(name1, name2);
        Ok(())
    }
}