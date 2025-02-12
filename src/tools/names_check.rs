use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;
use crate::RitmoErr;
use strsim::{jaro, jaro_winkler}; 
use unicode_normalization::UnicodeNormalization;  
use crate::ritmo_db::entities::people;
use rayon::prelude::*;

/// Normalize a name by converting to lowercase, removing diacritics, and trimming
fn normalize_name(name: &str) -> String {
    name.nfkd().collect::<String>().to_lowercase().trim().to_string()
}

async fn compare_names(names: Vec<String>, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    let normalized_names: Vec<String> = names
        .iter()
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

pub async fn check_names(path: &DatabaseConnection, jw_th: f64, j_th: f64) -> Result<Vec<(String, String, f64, f64)>, RitmoErr> {

    // query all names in people table
    let names: Vec<String> = get_names_list(path).await?;
    // Normalize names in parallel
    compare_names(names, jw_th, j_th).await
}

async fn get_names_list(path: &DatabaseConnection) -> Result<Vec<String>, RitmoErr> { // Use DbErr, or RitmoErr if it's defined as Result<..., DbErr>
    let people_result = people::Entity::find().all(path).await; // Store the Result

    match people_result {
        Ok(people) => {
            let names: Vec<String> = people.into_iter().map(|p| p.name).collect();
            Ok(names) // Return Ok with the collected names
        }
        Err(err) => Err(err.into()), // Return the error
    }
}

pub async fn compare_single_name(path: &DatabaseConnection, target_name: String, jaro_winkler_threshold: f64, jaro_threshold: f64) -> Result<Vec<(String, String)>, RitmoErr> {
    let names: Vec<String> = get_names_list(path).await?;

    // Normalize the target name
    let normalized_target = normalize_name(&target_name);
    
    // Find similar names   
    let similar_names: Vec<(String, String)> = names
        .iter()
        .filter_map(|name| {
            let normalized_name = normalize_name(name);
            
            // Skip comparing the name with itself
            if normalized_name == normalized_target {
                return None;
            }
            
            let similarity_jw = jaro_winkler(&normalized_target, &normalized_name);
            let similarity_j = jaro(&normalized_target, &normalized_name);

            if similarity_jw >= jaro_winkler_threshold && similarity_j >= jaro_threshold {
                Some((
                    target_name.clone(), 
                    name.clone(), 
                ))
            } else {
                None
            }
        })
        .collect();

    Ok(similar_names)
}