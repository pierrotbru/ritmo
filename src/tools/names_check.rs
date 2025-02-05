use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;
use crate::RitmoErr;
use strsim::{jaro, jaro_winkler}; 
use unicode_normalization::UnicodeNormalization;  
use crate::db::entity::people;

/// Normalize a name by converting to lowercase, removing diacritics, and trimming
fn normalize_name(name: &str) -> String {
    name.nfkd().collect::<String>().to_lowercase().trim().to_string()
}

async fn compare_names(names: Vec<String>, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    // Normalize names in parallel
    let normalized_names: Vec<String> = names
        .iter()
        .map(|name| normalize_name(name))
        .collect();

    // Find similar names in parallel
    let similar_names: Vec<(String, String, f64, f64)> = (0..names.len())
        .into_iter()
        .flat_map(|i| {
            (i + 1..names.len())
                .filter_map(|j| {
                    let similarity_jw = jaro_winkler(&normalized_names[i], &normalized_names[j]);
                    let similarity_j = jaro(&normalized_names[i], &normalized_names[j]);

                    if similarity_jw >= jaro_winkler_threshold && similarity_j >= jaro_threshold {
                        Some((
                            names[i].clone(), 
                            names[j].clone(), 
                            similarity_jw, 
                            similarity_j
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(similar_names)
}

pub async fn check_names(path: &DatabaseConnection, jw_th: f64, j_th: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    // query all names in people table
    let names: Vec<String> = people::Entity::find()
        .all(path)
        .await?
        .into_iter()
        .map(|p| p.name)
        .collect();

    compare_names(names, jw_th, j_th).await
}