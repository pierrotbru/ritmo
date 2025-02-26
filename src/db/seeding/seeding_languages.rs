use sqlx::{sqlite::SqlitePool, query};
use std::time::Instant;
use std::path::Path;
use csv::ReaderBuilder;
use crate::RitmoErr;

pub async fn get_languages_names() -> Result<Vec<(String, String)>, RitmoErr> {
    let path = Path::new("./resources/iso-639-2.tab");

    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            return Err(RitmoErr::PathError(format!("Error canonicalizing path: {}", e)));
        }
    };

    let mut reader = match ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path)
    {
        Ok(r) => r,
        Err(e) => return Err(RitmoErr::PathError(format!("CSV reader error: {}", e))),
    };

    let mut language_names = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => {
                let col0 = record.get(0);
                let col3 = record.get(3);

                match (col0, col3) {
                    (Some(val0), Some(val3)) => {
                        language_names.push((val0.to_string(), val3.to_string()));
                    }
                    _ => {
                        return Err(RitmoErr::PathError("Data record not found (column 0 or 3)".to_string()));
                    }
                }
            }
            Err(e) => return Err(RitmoErr::PathError(format!("CSV record error: {}", e))),
        }
    }
    Ok(language_names)
}

pub async fn seed_languages_names_table(pool: &SqlitePool) -> Result<(), RitmoErr> {
    let languages = get_languages_names().await?;

    if languages.is_empty() {
        return Ok(());
    }

    let start = Instant::now();

    let mut tx = pool.begin().await.map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    for (iso_code, language_name) in languages {
        query!("INSERT INTO languages_names (iso_code, name) VALUES (?, ?)", iso_code, language_name)
            .execute(&mut *tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
    }

    tx.commit().await.map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

    let duration = start.elapsed();
    println!("sqlx seeding languages: {:?}", duration);
    Ok(())
}
