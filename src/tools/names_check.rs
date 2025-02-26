use sqlx::SqlitePool;
use strsim::{jaro, jaro_winkler};
use unicode_normalization::UnicodeNormalization;
use rayon::prelude::*;
use crate::RitmoErr;

#[derive(sqlx::FromRow)]
struct Person {
    name: String,
}

fn normalize_name(name: &str) -> String {
    name.nfkd().collect::<String>().to_lowercase().trim().to_string()
}

async fn compare_names(names: Vec<String>, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {
    let normalized_names: Vec<String> = names.iter().map(|name| normalize_name(name)).collect();

    let similar_names: Vec<(String, String, f64, f64)> = (0..names.len())
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..names.len()).filter_map(|j| {
                let similarity_jw = jaro_winkler(&normalized_names[i], &normalized_names[j]);
                let similarity_j = jaro(&normalized_names[i], &normalized_names[j]);

                if similarity_jw >= jaro_winkler_threshold && similarity_j >= jaro_threshold {
                    Some((names[i].clone(), names[j].clone(), similarity_jw, similarity_j))
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        }).collect();

    Ok(similar_names)
}

pub async fn check_names(pool: &SqlitePool, jw_th: f64, j_th: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {
    let names: Vec<String> = get_names_list(pool).await?;
    compare_names(names, jw_th, j_th).await
}

async fn get_names_list(pool: &SqlitePool) -> Result<Vec<String>, RitmoErr> {
    let people_result: Vec<Person> = sqlx::query_as!(Person, "SELECT name FROM people")
        .fetch_all(pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    let names: Vec<String> = people_result.into_iter().map(|p| p.name).collect();
    Ok(names)
}

pub async fn compare_single_name(pool: &SqlitePool, target_name: String, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {
    let names: Vec<String> = get_names_list(pool).await?;
    let normalized_target = normalize_name(&target_name);

    let similar_names: Vec<(String, String, f64, f64)> = names
        .iter()
        .filter_map(|name| {
            let normalized_name = normalize_name(name);
            let similarity_jw = jaro_winkler(&normalized_target, &normalized_name);
            let similarity_j = jaro(&normalized_target, &normalized_name);

            if similarity_jw >= jaro_winkler_threshold && similarity_j >= jaro_threshold {
                Some((target_name.clone(), name.clone(), similarity_j, similarity_jw))
            } else {
                None
            }
        })
        .collect();

    Ok(similar_names)
}

