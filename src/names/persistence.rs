// src/name_matching/persistence.rs

use crate::names::names_ml::MLNameLearner;
use crate::names::utils::NameUtils;
use sqlx::{Row, Transaction, Sqlite, SqlitePool, query};
use crate::errors::RitmoErr;
use super::models::{PersonRecord, ParsedName};
use super::manager::NameManager;
 
impl NameManager {
    pub async fn load_ml_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        self.ml_learner = MLNameLearner::load_from_db(pool).await?;
        self.name_utils = NameUtils::load_from_db(pool).await?;
        Ok(())
    }

    /// Salva l'intera istanza di NameManager (e i suoi componenti) nel database.
    pub async fn save_ml_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        self.ml_learner.save_to_db(pool).await?;
        self.name_utils.save_to_db(pool).await?;
    
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn load_names_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, given_name, surname, middle_names, title, suffix, display_name, normalized_key, phonetic_key, confidence
            FROM people
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nel recupero dei nomi dal DB: {}", e)))?;

        for row in rows {
            let id: i64 = row.try_get("id")?;
            let original_input: String = row.try_get("name")?;
            let given_name: String = row.try_get("given_name")?;
            let surname: String = row.try_get("surname")?;
            let middle_names_str: Option<String> = row.try_get("middle_names")?;
            let middle_names: Vec<String> = middle_names_str
                .map(|s| s.split(',').filter(|part| !part.trim().is_empty()).map(|part| part.trim().to_string()).collect())
                .unwrap_or_default();
            let title: Option<String> = row.try_get("title")?;
            let suffix: Option<String> = row.try_get("suffix")?;
            let display_name: String = row.try_get("display_name")?;
            let normalized_key: String = row.try_get("normalized_key")?;
            let phonetic_key: String = row.try_get("phonetic_key")?;
            let confidence: f64 = row.try_get("confidence")?;
            let aliases: Vec<String> = Vec::new(); // Aliases should probably be loaded from a separate table if persisted
            let parsed_name = ParsedName {
                given_name,
                surname,
                middle_names,
                title,
                suffix,
                display_name,
            };
            let person_record = PersonRecord {
                id,
                original_input,
                parsed_name,
                normalized_key,
                phonetic_key,
                confidence,
                verified: true, // Assuming loaded from DB means verified
                aliases,
            };
            self.add_person_record(person_record)?;
        }
        Ok(())
    }

    async fn save_single_person_record_in_tx(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
        record: &PersonRecord,
    ) -> Result<(), RitmoErr> {
        let middle_names_str = if record.parsed_name.middle_names.is_empty() {
            None
        } else {
            Some(record.parsed_name.middle_names.join(", "))
        };
        let result = query(
            r#"
            INSERT OR REPLACE INTO people (
                id, name, given_name, surname, middle_names, title, suffix,
                display_name, normalized_key, phonetic_key, confidence, verified, created_at, updated_at, source
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'), ?)
            "#,
        )
        .bind(record.id)
        .bind(&record.original_input)
        .bind(&record.parsed_name.given_name)
        .bind(&record.parsed_name.surname)
        .bind(middle_names_str)
        .bind(&record.parsed_name.title)
        .bind(&record.parsed_name.suffix)
        .bind(&record.parsed_name.display_name)
        .bind(&record.normalized_key)
        .bind(&record.phonetic_key)
        .bind(record.confidence)
        .bind(record.verified)
        .bind("biblioteca")
        .execute(&mut **transaction)
        .await
        .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel salvare il record persona nel DB durante la transazione: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(RitmoErr::DatabaseTransactionError(format!("Nessuna riga modificata per ID {} durante il salvataggio del record.", record.id)));
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Salva nel DB i records di persone contenuti nel vettore records
    pub async fn save_person_records_to_db(
        &mut self,
        pool: &SqlitePool,
        records: &Vec<PersonRecord>,
        ) -> Result<(), RitmoErr> {
        let mut transaction = pool.begin()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nell'avviare la transazione: {}", e)))?;
        for record in records {
            self.save_single_person_record_in_tx(&mut transaction, &record).await?;
        }
        transaction.commit()
            .await
            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel commettere la transazione: {}", e)))?;

        self.train_ml_model()?; // Esegue il training del modello ML.
        self.save_ml_to_db(pool).await?;

        Ok(())
    }
//
//    /// Salva tutti i PersonRecord presenti nella HashMap `all_person_records` nel database, utilizzando una singola transazione.
//    pub async fn //save_manager_person_records_to_db(
//        &self,
//        pool: &SqlitePool,
//    ) -> Result<(), RitmoErr> {
//        let mut transaction = pool.begin()
//            .await
//            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nell'avviare la transazione: {}", e)))?;
//
//        for record in self.all_person_records.values() {
//            self.save_single_person_record_in_tx(&mut transaction, record).await?;
//        }
//        transaction.commit()
//            .await
//            .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel commettere la transazione: {}", e)))?;
//
//        self.save_ml_to_db(pool).await?;
//
//        Ok(())
//    }
}
