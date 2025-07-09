use sqlx::SqlitePool;
use crate::ml::{
    entity_learner::MLEntityLearner,
    entity_persistence::load_ml_from_db,
    generic::deduplicate_fuzzy,
    traits::MLProcessable,
};
use crate::ml::people::record::{PersonRecord, ParsedName};
use crate::ml::string_utils::MLStringUtils;
use crate::errors::RitmoErr;
use std::collections::HashMap;

/// Esempio di immissione di un nuovo nome nel sistema
pub async fn process_new_name_input(
    pool: &SqlitePool, 
    input_name: &str
) -> Result<ProcessingResult, RitmoErr> {
    println!("=== PROCESSAMENTO NUOVO NOME: '{}' ===", input_name);
    
    // 1. Inizializza il normalizzatore
    let normalizer = MLStringUtils::new(HashMap::new());
    
    // 2. Parsing e creazione del nuovo record
    let new_record = PersonRecord::new(0, input_name, &normalizer)?;
    println!("Nome parsato: {:?}", new_record.parsed_name);
    println!("Chiave normalizzata: '{}'", new_record.normalized_key);
    
    // 3. Carica i dati ML esistenti
    let ml_data = load_ml_from_db(pool, "people").await?;
    println!("Caricati {} cluster e {} pattern ML", 
             ml_data.clusters.len(), ml_data.learned_patterns.len());
    
    // 4. Carica i nomi esistenti dal database
    let existing_records = load_existing_people(pool).await?;
    println!("Caricati {} nomi esistenti dal database", existing_records.len());
    
    // 5. Controlla duplicati usando i pattern ML
    let ml_matches = check_ml_patterns(&new_record, &ml_data);
    println!("Match ML trovati: {}", ml_matches.len());
    
    // 6. Controlla duplicati nei nomi esistenti
    let existing_matches = check_existing_names(&new_record, &existing_records);
    println!("Match esistenti trovati: {}", existing_matches.len());
    
    // 7. Determina l'azione da intraprendere
    let result = determine_action(new_record, ml_matches, existing_matches);
    
    // 8. Esegui l'azione
    match &result.action {
        ProcessingAction::Insert => {
            println!("✓ Nome nuovo, inserimento confermato");
            insert_new_person(pool, &result.record).await?;
        }
        ProcessingAction::Duplicate(existing_id) => {
            println!("⚠ Duplicato rilevato con ID: {}", existing_id);
            // Opzionalmente aggiungi come alias
        }
        ProcessingAction::RequiresVerification => {
            println!("? Richiesta verifica manuale");
            // Salva in tabella pending_verification
        }
    }
    
    Ok(result)
}

/// Struttura per il risultato del processamento
#[derive(Debug)]
pub struct ProcessingResult {
    pub record: PersonRecord,
    pub action: ProcessingAction,
    pub ml_matches: Vec<MLMatch>,
    pub existing_matches: Vec<ExistingMatch>,
}

#[derive(Debug)]
pub enum ProcessingAction {
    Insert,
    Duplicate(i64), // ID del record duplicato
    RequiresVerification,
}

#[derive(Debug)]
pub struct MLMatch {
    pub matched_key: String,
    pub confidence: f64,
    pub pattern_type: String,
}

#[derive(Debug)]
pub struct ExistingMatch {
    pub record_id: i64,
    pub matched_name: String,
    pub similarity: f64,
}

/// Carica i nomi esistenti dal database
async fn load_existing_people(pool: &SqlitePool) -> Result<Vec<PersonRecord>, RitmoErr> {
    let rows = sqlx::query!(
        "SELECT id, original_input, parsed_name_json, normalized_key, confidence, verified 
         FROM people"
    )
    .fetch_all(pool)
    .await?;
    
    let mut records = Vec::new();
    for row in rows {
        let parsed_name: ParsedName = serde_json::from_str(&row.parsed_name_json)?;
        records.push(PersonRecord {
            id: row.id,
            original_input: row.original_input,
            parsed_name,
            normalized_key: row.normalized_key,
            confidence: row.confidence,
            verified: row.verified,
            aliases: Vec::new(), // Carica separatamente se necessario
        });
    }
    
    Ok(records)
}

/// Controlla i pattern ML per possibili match
fn check_ml_patterns(
    new_record: &PersonRecord, 
    ml_data: &MLEntityLearner
) -> Vec<MLMatch> {
    let mut matches = Vec::new();
    
    // Controlla i cluster
    for cluster in &ml_data.clusters {
        for member in &cluster.members {
            let similarity = strsim::jaro_winkler(&new_record.normalized_key, member);
            if similarity > 0.85 {
                matches.push(MLMatch {
                    matched_key: member.clone(),
                    confidence: similarity * cluster.confidence,
                    pattern_type: "cluster".to_string(),
                });
            }
        }
    }
    
    // Controlla i pattern appresi
    for pattern in &ml_data.learned_patterns {
        let base_similarity = strsim::jaro_winkler(&new_record.normalized_key, &pattern.base_form);
        let variant_similarity = strsim::jaro_winkler(&new_record.normalized_key, &pattern.variant_form);
        
        if base_similarity > 0.85 || variant_similarity > 0.85 {
            matches.push(MLMatch {
                matched_key: if base_similarity > variant_similarity { 
                    pattern.base_form.clone() 
                } else { 
                    pattern.variant_form.clone() 
                },
                confidence: pattern.confidence,
                pattern_type: format!("{:?}", pattern.pattern_type),
            });
        }
    }
    
    matches
}

/// Controlla i nomi esistenti per possibili match
fn check_existing_names(
    new_record: &PersonRecord, 
    existing_records: &[PersonRecord]
) -> Vec<ExistingMatch> {
    let mut matches = Vec::new();
    
    for existing in existing_records {
        let similarity = strsim::jaro_winkler(
            &new_record.normalized_key, 
            &existing.normalized_key
        );
        
        if similarity > 0.85 {
            matches.push(ExistingMatch {
                record_id: existing.id,
                matched_name: existing.original_input.clone(),
                similarity,
            });
        }
        
        // Controlla anche gli alias
        for alias in &existing.aliases {
            let alias_similarity = strsim::jaro_winkler(&new_record.normalized_key, alias);
            if alias_similarity > 0.85 {
                matches.push(ExistingMatch {
                    record_id: existing.id,
                    matched_name: format!("{} (alias: {})", existing.original_input, alias),
                    similarity: alias_similarity,
                });
            }
        }
    }
    
    matches
}

/// Determina l'azione da intraprendere
fn determine_action(
    record: PersonRecord,
    ml_matches: Vec<MLMatch>,
    existing_matches: Vec<ExistingMatch>
) -> ProcessingResult {
    // Trova il match esistente con similarità più alta
    let best_existing = existing_matches.iter()
        .max_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap());
    
    // Trova il match ML con confidence più alta
    let best_ml = ml_matches.iter()
        .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
    
    let action = match (best_existing, best_ml) {
        // Match esatto nei nomi esistenti
        (Some(existing), _) if existing.similarity > 0.95 => {
            ProcessingAction::Duplicate(existing.record_id)
        }
        
        // Match ML molto alto
        (_, Some(ml)) if ml.confidence > 0.95 => {
            ProcessingAction::RequiresVerification
        }
        
        // Match moderato - richiede verifica
        (Some(existing), _) if existing.similarity > 0.85 => {
            ProcessingAction::RequiresVerification
        }
        
        // Nessun match significativo - inserisci
        _ => ProcessingAction::Insert,
    };
    
    ProcessingResult {
        record,
        action,
        ml_matches,
        existing_matches,
    }
}

/// Inserisce un nuovo record nel database
async fn insert_new_person(pool: &SqlitePool, record: &PersonRecord) -> Result<i64, RitmoErr> {
    let parsed_name_json = serde_json::to_string(&record.parsed_name)?;
    
    let result = sqlx::query!(
        "INSERT INTO people (original_input, parsed_name_json, normalized_key, confidence, verified) 
         VALUES (?, ?, ?, ?, ?)",
        record.original_input,
        parsed_name_json,
        record.normalized_key,
        record.confidence,
        record.verified
    )
    .execute(pool)
    .await?;
    
    Ok(result.last_insert_rowid())
}

/// Esempio di utilizzo
pub async fn example_usage(pool: &SqlitePool) -> Result<(), RitmoErr> {
    // Esempi di input diversi
    let test_names = vec![
        "Stephen King",           // Nuovo nome
        "Steven King",            // Typo di Stephen King
        "S. King",               // Abbreviazione
        "Dr. Stephen King Jr.",   // Con titolo e suffisso
        "King, Stephen",          // Formato diverso
        "Madonna",               // Nome singolo
        "J.K. Rowling",          // Nome completamente nuovo
    ];
    
    for name in test_names {
        println!("\n" + "=".repeat(50).as_str());
        let result = process_new_name_input(pool, name).await?;
        
        match result.action {
            ProcessingAction::Insert => {
                println!("✓ '{}' inserito come nuovo nome", name);
            }
            ProcessingAction::Duplicate(id) => {
                println!("⚠ '{}' è un duplicato di ID: {}", name, id);
            }
            ProcessingAction::RequiresVerification => {
                println!("? '{}' richiede verifica manuale", name);
                println!("  ML matches: {:?}", result.ml_matches);
                println!("  Existing matches: {:?}", result.existing_matches);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    
    #[tokio::test]
    async fn test_new_name_processing() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        
        // Setup database
        setup_test_db(&pool).await.unwrap();
        
        // Test con nome nuovo
        let result = process_new_name_input(&pool, "Stephen King").await.unwrap();
        assert!(matches!(result.action, ProcessingAction::Insert));
        
        // Test con possibile duplicato
        let result = process_new_name_input(&pool, "Steven King").await.unwrap();
        println!("Result: {:?}", result);
    }
    
    async fn setup_test_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(r#"
            CREATE TABLE people (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_input TEXT NOT NULL,
                parsed_name_json TEXT NOT NULL,
                normalized_key TEXT NOT NULL,
                confidence REAL NOT NULL,
                verified BOOLEAN NOT NULL
            )
        "#)
        .execute(pool)
        .await?;
        
        sqlx::query(r#"
            CREATE TABLE ml_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data_type TEXT NOT NULL UNIQUE,
                data_json TEXT NOT NULL
            )
        "#)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}