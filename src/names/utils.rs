// src/name_matching/utils.rs

use sqlx::Row;
use sqlx::SqlitePool;
use std::collections::HashMap;
use human_name::Name;
use strsim::levenshtein;
use unicode_normalization::UnicodeNormalization;
use rphonetic::{DoubleMetaphone, Encoder};
use crate::errors::RitmoErr;
use super::models::{NameManagerErrorInternal, ParsedName}; // Importa i tipi dal modulo models

pub struct NameUtils {
    pub double_metaphone: DoubleMetaphone,
    pub name_variants: std::collections::HashMap<String, Vec<String>>, // Usato da are_known_variants
}

impl NameUtils {
    pub fn new(double_metaphone: DoubleMetaphone, name_variants: std::collections::HashMap<String, Vec<String>>) -> Self {
        NameUtils {
            double_metaphone,
            name_variants,
        }
    }

    async fn load_data<T: for<'de> serde::Deserialize<'de>>(
        pool: &SqlitePool,
        data_type: &str,
    ) -> Result<Option<T>, RitmoErr> {
        let row = sqlx::query("SELECT data_json FROM ml_name_data WHERE data_type = ?")
            .bind(data_type)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(r) => {
                let json_string: String = r.try_get("data_json")?;
                let data: T = serde_json::from_str(&json_string)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    async fn save_data<T: serde::Serialize>(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        data_type: &str,
        data_value: &T,
    ) -> Result<(), RitmoErr> {
        let json_string = serde_json::to_string(data_value)?;
        
        sqlx::query("INSERT OR REPLACE INTO ml_name_data (id, data_type, data_json)
                      VALUES ((SELECT id FROM ml_name_data WHERE data_type = ?), ?, ?)")
            .bind(data_type)
            .bind(data_type)
            .bind(json_string)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    /// Carica un'istanza di NameUtils dal database.
    pub async fn load_from_db(pool: &SqlitePool) -> Result<Self, RitmoErr> {
        let name_variants = Self::load_data(pool, "name_variants")
            .await?
            .unwrap_or_else(HashMap::new); // Se non trova, inizia con una HashMap vuota

        let double_metaphone = Self::load_data(pool, "double_metaphone").await?;

        Ok(NameUtils {
            name_variants,
            double_metaphone: double_metaphone.expect("REASON")
        })
    }

    /// Salva NameUtils nel database.
    pub async fn save_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let mut tx = pool.begin().await?;

        Self::save_data(&mut tx, "name_variants", &self.name_variants).await?;
        // Salva altri campi di NameUtils se presenti con Self::save_data

        tx.commit().await?;
        Ok(())
    }

    pub fn normalize_string(&self, text: &str) -> String {
        let normalized = text
            .nfc()
            .collect::<String>()
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
                'è' | 'é' | 'ê' | 'ë' => 'e',
                'ì' | 'í' | 'î' | 'ï' => 'i',
                'ō' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
                'ù' | 'ú' | 'û' | 'ü' => 'u',
                'ç' => 'c',
                'ñ' => 'n',
                'ý' | 'ÿ' => 'y',
                'č' | 'ć' => 'c',
                'š' => 's',
                'ž' => 'z',
                'đ' => 'd',
                'ł' => 'l',
                c if c.is_alphabetic() || c.is_whitespace() => c,
                _ => ' ',
            })
            .collect::<String>();

        normalized.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    pub fn generate_phonetic_key(&self, text: &str) -> String {
        let normalized = self.normalize_string(text);
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        let mut phonetic_parts = Vec::new();

        for part in parts {
            phonetic_parts.push(self.double_metaphone.encode(part));
        }

        phonetic_parts.join(" ")
    }

    pub fn normalized_levenshtein_distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            return 1.0;
        }
        1.0 - (levenshtein(s1, s2) as f64 / max_len)
    }

    pub fn are_known_variants(&self, name1: &str, name2: &str) -> bool {
        let norm1 = self.normalize_string(name1);
        let norm2 = self.normalize_string(name2);

        if let Some(variants) = self.name_variants.get(&norm1) {
            if variants.contains(&norm2) {
                return true;
            }
        }

        if let Some(variants) = self.name_variants.get(&norm2) {
            if variants.contains(&norm1) {
                return true;
            }
        }

        false
    }

    pub fn parse_name(&self, input: &str) -> Result<ParsedName, RitmoErr> {
        let parsed: Name = Name::parse(input)
            .ok_or_else(|| NameManagerErrorInternal::NameParsingError(format!("Impossibile parsificare il nome: '{}'", input)))?;

        let given_name = parsed.given_name().unwrap_or("").to_string();
        let surname = parsed.surname().to_string();
        let middle_names: Vec<String> = parsed.middle_names()
            .map(|names| names.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let title = parsed.honorific_prefix().map(|s| s.to_string());
        let suffix = parsed.generational_suffix().map(|s| s.to_string());
        let display_name = parsed.display_first_last();

        Ok(ParsedName {
            given_name,
            surname,
            middle_names,
            title,
            suffix,
            display_name: display_name.to_string(),
        })
    }

    pub fn normalize_parsed_name_for_matching(&self, parsed_name: &ParsedName) -> String {
        let mut full_name_parts = Vec::new();

        if !parsed_name.given_name.is_empty() {
            full_name_parts.push(parsed_name.given_name.as_str());
        }

        for middle_name in &parsed_name.middle_names {
            if !middle_name.is_empty() {
                full_name_parts.push(middle_name.as_str());
            }
        }

        if !parsed_name.surname.is_empty() {
            full_name_parts.push(parsed_name.surname.as_str());
        }

        let combined_name = full_name_parts.join(" ");
        self.normalize_string(&combined_name)
    }
}