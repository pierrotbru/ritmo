use crate::filters::constants::constants::*;
use sqlx::{SqlitePool, FromRow};
use crate::errors::RitmoErr;
use strsim::{jaro, jaro_winkler};
use unicode_normalization::UnicodeNormalization;
use rayon::prelude::*;
use crate::db::query_builder::QueryBuilder;

/// Normalize a name by converting to lowercase, removing diacritics, and trimming
fn normalize_name(name: &str) -> String {
    name.nfkd().collect::<String>().to_lowercase().trim().to_string()
}

#[derive(FromRow)]
struct NameRow {
    name: String,
}

/// Check for similar names in the database with configurable similarity thresholds
pub async fn check_names (
    pool: &SqlitePool, 
    jaro_winkler_threshold: f64, 
    jaro_threshold: f64
) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    let query = QueryBuilder::new(TABLE_PEOPLE)
        .select_columns(&[COLUMN_PEOPLE_NAME]);

    let t_names = query.execute::<NameRow>(pool).await?;
    let names: Vec<String> = t_names.collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|row| row.name)
        .collect();

    println!("i nomi sono {:?}",names.len() );
    compare_names(names, jaro_threshold, jaro_winkler_threshold).await
}

pub async fn check_publ (
    pool: &SqlitePool, 
    jaro_winkler_threshold: f64, 
    jaro_threshold: f64
) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    let query = QueryBuilder::new(TABLE_PUBLISHERS)
        .select_columns(&[COLUMN_PUBLISHERS_NAME]);

    let t_names = query.execute::<NameRow>(pool).await?;
    let names: Vec<String> = t_names.collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|row| row.name)
        .collect();

    println!("i nomi sono {:?}",names.len() );
    compare_names(names, jaro_threshold, jaro_winkler_threshold).await
}

async fn compare_names(names: Vec<String>, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    // Normalize names in parallel
    let normalized_names: Vec<String> = names
        .par_iter()
        .map(|name| normalize_name(name))
        .collect();

    // Find similar names in parallel
    let similar_names: Vec<(String, String, f64, f64)> = (0..names.len())
        .into_par_iter()
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