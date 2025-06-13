use sqlx::SqlitePool;
use crate::errors::RitmoErr;

pub enum FeedbackType {
    FalsePositive,
    FalseNegative,
}

impl ToString for FeedbackType {
    fn to_string(&self) -> String {
        match self {
            FeedbackType::FalsePositive => "false_positive".to_string(),
            FeedbackType::FalseNegative => "false_negative".to_string(),
        }
    }
}

/// Salva un feedback nel database
pub async fn save_feedback(
    pool: &SqlitePool,
    feedback_type: FeedbackType,
    name1: &str,
    name2: &str,
) -> Result<(), RitmoErr> {
    sqlx::query(
        r#"
        INSERT INTO ml_feedback (feedback_type, name1, name2) VALUES (?, ?, ?)
        "#,
    )
    .bind(feedback_type.to_string())
    .bind(name1)
    .bind(name2)
    .execute(pool)
    .await?;
    Ok(())
}