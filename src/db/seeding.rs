use std::time::Instant;
use sqlx::SqlitePool;
use std::path::Path;
use csv::ReaderBuilder;
use crate::errors::RitmoErr;

use sea_orm::{
    DbErr,
    EntityTrait,
    Set,
    SqlxSqliteConnector
};
use crate::db::entity::prelude::*;

pub async fn seed_all(pool: &SqlitePool) -> Result<(), RitmoErr> {
    
    let _ = seed_languages_names_table(pool).await?;
    let _ = seed_laverdure_table(pool).await?;
    let _ = seed_content_types(pool).await?;
    let _ = seed_roles(pool).await?;

    Ok(())
}

async fn get_languages_names() -> Result<Vec<(String, String)>, DbErr> {
    let path = Path::new("./resources/iso-639-2.tab");

    // More robust path handling:
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            // Convert the io::Error to DbErr.  You might want a more specific DbErr variant.
            return Err(DbErr::Custom(format!("Error canonicalizing path: {}", e)));
        }
    };

    let mut reader = match ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false) // Se il tuo file non ha intestazioni
        .from_path(path) {
            Ok(r) => r,
            Err(e) => return Err(DbErr::Custom(format!("CSV reader error: {}", e))),
    };

    let mut language_names = Vec::new(); // Pre-allocate for efficiency

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
                        return Err(DbErr::Custom("Dati mancanti nel record (colonna 0 o 3)".to_string()));
                    }
                }
            }
            Err(e) => return Err(DbErr::Custom(format!("Errore leggendo un record CSV: {}", e))),
        }
    }

    Ok(language_names)
}

async fn seed_languages_names_table(pool: &SqlitePool) -> Result<(), RitmoErr> {

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

    let languages = get_languages_names().await?;

    if languages.is_empty() {
        return Ok(()); // Nothing to insert
    }
    let start = Instant::now();

    let mut iso_table : Vec<crate::db::entity::languages_names::ActiveModel> = Vec::new();

    for (iso_code, language_name) in languages {
        let iso_single: crate::db::entity::languages_names::ActiveModel = crate::db::entity::languages_names::ActiveModel {
            ref_name: Set(Some(language_name.to_owned())),
            id: Set(iso_code.to_owned()),
            ..Default::default()
        };
        iso_table.push(iso_single);
    }

    let _ = LanguagesNames::insert_many(iso_table).exec(&conn).await?;

    let duration = start.elapsed();
    println!("SeaORM seeding languages: {:?}", duration);
    Ok(())
}

async fn seed_laverdure_table(pool: &SqlitePool) -> Result<(), DbErr> {
    let laverdure_data = vec![
        ("author", "laverdure"),
        ("program", "ritmo"),
        ("program release", "0.0.0"),
        ("database_version", "0.0.0"),
    ];

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

    let start = Instant::now();

    let mut table : Vec<crate::db::entity::laverdure::ActiveModel> = Vec::new();
    for (key, value) in laverdure_data {
        let single: crate::db::entity::laverdure::ActiveModel = crate::db::entity::laverdure::ActiveModel {
            key: Set(key.to_string()),
            value: Set(Some(value.to_string())),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Laverdure::insert_many(table).exec(&conn).await?;

    let duration = start.elapsed();
    println!("SeaORM seeding laverdure: {:?}", duration);
    Ok(())
}

async fn seed_content_types(pool: &SqlitePool) -> Result<(), DbErr> {
    let types_data = vec![
        "Novel",
        "Short novel",
        "Short story",
        "Essay",
        "Treatise",
        "Dissertation",
        "Biography",
        "Autobiography",
        "Memoir",
        "Interview",
        "Play",
        "One-act play",
        "Poetry",
        "Opera"
    ];

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

    let start = Instant::now();

    let mut table : Vec<crate::db::entity::contents_types::ActiveModel> = Vec::new();
    for name in types_data {
        let single: crate::db::entity::contents_types::ActiveModel = crate::db::entity::contents_types::ActiveModel {
            type_name: Set(name.to_string()),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = ContentsTypes::insert_many(table).exec(&conn).await?;

    let duration = start.elapsed();
    println!("SeaORM seeding contents_types: {:?}", duration);
    Ok(())
}

async fn seed_roles(pool: &SqlitePool) -> Result<(), DbErr> {
    let types_data = vec![
        "Author",
        "Translator",
        "Curator",
        "Cover designer",
        "Commentator",
        "Interviewer",
        "Reporter",
        "Photographer",
        "Comic book artist",
    ];

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

    let start = Instant::now();

    let mut table : Vec<crate::db::entity::roles::ActiveModel> = Vec::new();
    for name in types_data {
        let single: crate::db::entity::roles::ActiveModel = crate::db::entity::roles::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Roles::insert_many(table).exec(&conn).await?;

    let duration = start.elapsed();
    println!("SeaORM seeding roles: {:?}", duration);
    Ok(())
}

