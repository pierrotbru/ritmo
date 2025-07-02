use sqlx::SqlitePool;
use chrono::Utc;
use crate::RitmoErr;

#[derive(Debug, Clone, Copy)]
pub enum FeedbackType {
    FalsePositive,
    FalseNegative,
}

pub async fn save_feedback(
    pool: &SqlitePool,
    feedback_type: FeedbackType,
    publisher1: &str,
    publisher2: &str
) -> Result<(), RitmoErr> {
    let feedback_type_str = match feedback_type {
        FeedbackType::FalsePositive => "false_positive",
        FeedbackType::FalseNegative => "false_negative",
    };
    let timestamp = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO ml_publisher_feedback (feedback_type, publisher1, publisher2, timestamp)
         VALUES (?, ?, ?, ?)"
    )
    .bind(feedback_type_str)
    .bind(publisher1)
    .bind(publisher2)
    .bind(timestamp)
    .execute(pool)
    .await?;

    Ok(())
}