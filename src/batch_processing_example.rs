use std::collections::HashMap;
use sqlx::SqlitePool;
use crate::ml::{
    entity_learner::{MLEntityLearner, VariantPatternType},
    entity_persistence::{save_ml_to_db, load_ml_from_db},
    feedback::Feedback,
    generic::{cluster_by_canonical_key, deduplicate_fuzzy, apply_negative_feedback},
    traits::MLProcessable,
    utils::MLStringUtils,
};
use crate::people::record::PersonRecord;
use crate::tags::record::TagRecord;
use crate::publishers::record::PublisherRecord;
use crate::series::record::SeriesRecord;
use human_name::Name;
use crate::errors::RitmoErr;

// Aggiorniamo PersonRecord per usare human_name nel parsing iniziale
impl PersonRecord {
    pub fn new_with_parsing(id: i64, full_name: &str) -> Self {
        // Usa human_name per il parsing iniziale
        if let Some(parsed) = Name::parse(full_name) {
            let normalized_name = format!("{} {}", 
                parsed.given_name().unwrap_or("").trim(),
                parsed.surname().trim()
            ).trim().to_lowercase();
            
            PersonRecord {
                id,
                full_name: full_name.to_string(),
                normalized_name,
                given_name: parsed.given_name().map(|s| s.to_string()),
                surname: Some(parsed.surname().to_string()),
            }
        } else {
            // Fallback se il parsing fallisce
            PersonRecord::new(id, full_name)
        }
    }
}
impl MLProcessable for PersonRecord {
    fn id(&self) -> i64 {
        self.id
    }

    fn canonical_key(&self) -> &str {
        &self.normalized_name
    }

    fn variants(&self) -> Vec<String> {
        // In un'implementazione reale, questi potrebbero essere caricati dal DB
        vec![self.full_name.clone()]
    }

    fn set_variants(&mut self, variants: Vec<String>) {
        // In un'implementazione reale, salveresti sul DB
        println!("Aggiornando varianti per {}: {:?}", self.full_name, variants);
    }
}

// Implementazione del trait MLProcessable per TagRecord
impl MLProcessable for TagRecord {
    fn id(&self) -> i64 {
        self.id
    }

    fn canonical_key(&self) -> &str {
        &self.normalized_label
    }

    fn variants(&self) -> Vec<String> {
        vec![self.label.clone()]
    }

    fn set_variants(&mut self, variants: Vec<String>) {
        println!("Aggiornando varianti per tag {}: {:?}", self.label, variants);
    }
}

/// Esempio di caricamento e processamento batch per nomi di persone
pub async fn process_people_batch(pool: &SqlitePool) -> Result<(), RitmoErr> {
    println!("=== PROCESSAMENTO BATCH NOMI ===");
    
    // Simula il caricamento di un batch di nomi dal database
    // Usa il nuovo metodo con parsing human_name
    let mut people_records = vec![
        PersonRecord::new_with_parsing(1, "Stephen King"),
        PersonRecord::new_with_parsing(2, "Steven King"),  // Variante comune
        PersonRecord::new_with_parsing(3, "S. King"),     // Abbreviazione
        PersonRecord::new_with_parsing(4, "King, Stephen"), // Formato cognome, nome
        PersonRecord::new_with_parsing(5, "J.K. Rowling"),
        PersonRecord::new_with_parsing(6, "Joanne Rowling"),
        PersonRecord::new_with_parsing(7, "J. K. Rowling"),
        PersonRecord::new_with_parsing(8, "Isaac Asimov"),
        PersonRecord::new_with_parsing(9, "I. Asimov"),
        PersonRecord::new_with_parsing(10, "George Orwell"),
        PersonRecord::new_with_parsing(11, "Eric Blair"),  // Nome reale di Orwell
        PersonRecord::new_with_parsing(12, "Dame Agatha Christie"),
        PersonRecord::new_with_parsing(13, "A. Christie"),
        PersonRecord::new_with_parsing(14, "Dr. Martin Luther King Jr."),
        PersonRecord::new_with_parsing(15, "Martin Luther King"),
    ];

    // Stampa alcuni esempi di parsing per verificare
    println!("Esempi di parsing con human_name:");
    for person in people_records.iter().take(5) {
        println!("  {} -> normalized: '{}' | given: '{:?}' | surname: '{:?}'", 
                 person.full_name, person.normalized_name, 
                 person.given_name, person.surname);
    }
    println!();

    // Inizializza l'ML learner
    let mut ml_learner = MLEntityLearner::new();
    
    // Crea i cluster iniziali
    let names: Vec<String> = people_records.iter()
        .map(|p| p.canonical_key().to_string())
        .collect();
    
    ml_learner.create_clusters(&names);
    println!("Creati {} cluster", ml_learner.clusters.len());

    // Stampa i cluster trovati
    for (i, cluster) in ml_learner.clusters.iter().enumerate() {
        println!("Cluster {}: {} (confidence: {:.2})", 
                 i + 1, cluster.centroid, cluster.confidence);
        for member in &cluster.members {
            println!("  - {}", member);
        }
    }

    // Definisce le funzioni per classificare i pattern specifici per i nomi
    let classify_pattern = |a: &str, b: &str, edit_dist: usize| -> VariantPatternType {
        // Controlla se uno è abbreviazione dell'altro
        if (a.len() < b.len() && is_abbreviation(a, b)) || 
           (b.len() < a.len() && is_abbreviation(b, a)) {
            VariantPatternType::Abbreviation
        } else if a.contains(',') || b.contains(',') {
            VariantPatternType::Other // Formato "Cognome, Nome"
        } else if edit_dist <= 2 && strsim::jaro_winkler(a, b) > 0.85 {
            VariantPatternType::Typo
        } else if are_name_variants(a, b) {
            VariantPatternType::Other // Varianti conosciute come "George Orwell" / "Eric Blair"
        } else {
            VariantPatternType::Other
        }
    };

    let calculate_confidence = |a: &str, b: &str, pattern_type: &VariantPatternType, base_sim: f64| -> f64 {
        match pattern_type {
            VariantPatternType::Abbreviation => base_sim * 0.95,
            VariantPatternType::Typo => base_sim * 0.90,
            _ => base_sim * 0.85,
        }
    };

    // Identifica i pattern di varianti
    ml_learner.identify_variant_patterns(&classify_pattern, &calculate_confidence);
    println!("Identificati {} pattern di varianti", ml_learner.learned_patterns.len());

    // Stampa i pattern appresi
    for pattern in &ml_learner.learned_patterns {
        println!("Pattern: {} -> {} ({:?}, confidence: {:.2})", 
                 pattern.base_form, pattern.variant_form, 
                 pattern.pattern_type, pattern.confidence);
    }

    // Applica il clustering ai record
    cluster_by_canonical_key(&mut people_records);

    // Applica deduplica fuzzy
    deduplicate_fuzzy(&mut people_records, |a, b| {
        strsim::jaro_winkler(a, b) > 0.85
    });

    // Salva i risultati ML
    let mut tx = pool.begin().await?;
    save_ml_to_db(&mut tx, &ml_learner, "people").await?;
    tx.commit().await?;

    println!("Dati ML salvati per le persone");
    Ok(())
}

/// Esempio di caricamento e processamento batch per tag
pub async fn process_tags_batch(pool: &SqlitePool) -> Result<(), RitmoErr> {
    println!("\n=== PROCESSAMENTO BATCH TAGS ===");
    
    // Simula il caricamento di un batch di tag dal database
    let mut tag_records = vec![
        TagRecord::new(1, "Science Fiction"),
        TagRecord::new(2, "Sci-Fi"),
        TagRecord::new(3, "SciFi"),
        TagRecord::new(4, "Fantasy"),
        TagRecord::new(5, "Epic Fantasy"),
        TagRecord::new(6, "High Fantasy"),
        TagRecord::new(7, "Mystery"),
        TagRecord::new(8, "Mystery & Suspense"),
        TagRecord::new(9, "Thriller"),
        TagRecord::new(10, "Crime"),
        TagRecord::new(11, "True Crime"),
        TagRecord::new(12, "Horror"),
        TagRecord::new(13, "Psychological Horror"),
        TagRecord::new(14, "Romance"),
        TagRecord::new(15, "Romantic Fiction"),
        TagRecord::new(16, "Historical Fiction"),
        TagRecord::new(17, "Historical"),
    ];

    println!("Caricati {} record di tag", tag_records.len());

    // Inizializza l'ML learner per i tag
    let mut ml_learner = MLEntityLearner::new();
    ml_learner.minimum_confidence = 0.80; // Soglia più bassa per i tag
    
    // Crea i cluster
    let tags: Vec<String> = tag_records.iter()
        .map(|t| t.canonical_key().to_string())
        .collect();
    
    ml_learner.create_clusters(&tags);
    println!("Creati {} cluster di tag", ml_learner.clusters.len());

    // Stampa i cluster trovati
    for (i, cluster) in ml_learner.clusters.iter().enumerate() {
        println!("Cluster Tag {}: {} (confidence: {:.2})", 
                 i + 1, cluster.centroid, cluster.confidence);
        for member in &cluster.members {
            println!("  - {}", member);
        }
    }

    // Funzioni specifiche per i tag
    let classify_tag_pattern = |a: &str, b: &str, edit_dist: usize| -> VariantPatternType {
        if a.contains(&b) || b.contains(&a) {
            if a.len() < b.len() {
                VariantPatternType::Suffix // "Fantasy" vs "Epic Fantasy"
            } else {
                VariantPatternType::Prefix
            }
        } else if a.replace(|c: char| !c.is_alphanumeric(), "") == 
                  b.replace(|c: char| !c.is_alphanumeric(), "") {
            VariantPatternType::Abbreviation // "Sci-Fi" vs "SciFi"
        } else if edit_dist <= 2 {
            VariantPatternType::Typo
        } else {
            VariantPatternType::Other
        }
    };

    let calculate_tag_confidence = |a: &str, b: &str, pattern_type: &VariantPatternType, base_sim: f64| -> f64 {
        match pattern_type {
            VariantPatternType::Abbreviation => base_sim * 0.95,
            VariantPatternType::Suffix | VariantPatternType::Prefix => base_sim * 0.90,
            VariantPatternType::Typo => base_sim * 0.85,
            _ => base_sim * 0.80,
        }
    };

    // Identifica i pattern
    ml_learner.identify_variant_patterns(&classify_tag_pattern, &calculate_tag_confidence);
    println!("Identificati {} pattern di tag", ml_learner.learned_patterns.len());

    // Stampa i pattern appresi per i tag
    for pattern in &ml_learner.learned_patterns {
        println!("Pattern Tag: {} -> {} ({:?}, confidence: {:.2})", 
                 pattern.base_form, pattern.variant_form, 
                 pattern.pattern_type, pattern.confidence);
    }

    // Applica il clustering
    cluster_by_canonical_key(&mut tag_records);

    // Deduplica fuzzy con soglia più permissiva per i tag
    deduplicate_fuzzy(&mut tag_records, |a, b| {
        strsim::jaro_winkler(a, b) > 0.75
    });

    // Esempio di feedback negativo per i tag
    let mut feedback = Feedback::new();
    feedback.add_negative("fantasy", "sciencefiction"); // Non unire Fantasy e Sci-Fi
    feedback.add_negative("horror", "romance"); // Non unire Horror e Romance
    
    apply_negative_feedback(&mut tag_records, &feedback.negative_pairs);

    // Salva i risultati ML per i tag
    let mut tx = pool.begin().await?;
    save_ml_to_db(&mut tx, &ml_learner, "tags").await?;
    tx.commit().await?;

    println!("Dati ML salvati per i tag");
    Ok(())
}

/// Esempio di caricamento dei dati ML salvati
pub async fn load_saved_ml_data(pool: &SqlitePool) -> Result<(), RitmoErr> {
    println!("\n=== CARICAMENTO DATI ML SALVATI ===");
    
    // Carica i dati ML delle persone
    let people_ml = load_ml_from_db(pool, "people").await?;
    println!("Caricati dati ML persone: {} cluster, {} pattern", 
             people_ml.clusters.len(), people_ml.learned_patterns.len());

    // Carica i dati ML dei tag
    let tags_ml = load_ml_from_db(pool, "tags").await?;
    println!("Caricati dati ML tag: {} cluster, {} pattern", 
             tags_ml.clusters.len(), tags_ml.learned_patterns.len());

    Ok(())
}

/// Funzione principale di esempio
fn is_abbreviation(short: &str, long: &str) -> bool {
    let short_parts: Vec<&str> = short.split_whitespace().collect();
    let long_parts: Vec<&str> = long.split_whitespace().collect();
    
    if short_parts.len() != long_parts.len() {
        return false;
    }
    
    for (s, l) in short_parts.iter().zip(long_parts.iter()) {
        if s.len() == 1 {
            // Controllo iniziale: "S." dovrebbe matchare "Stephen"
            if !l.starts_with(s.chars().next().unwrap()) {
                return false;
            }
        } else if s != l {
            return false;
        }
    }
    true
}

fn are_name_variants(a: &str, b: &str) -> bool {
    // Qui potresti implementare controlli per pseudonimi conosciuti
    // Per ora un controllo semplice
    let known_variants = [
        ("george orwell", "eric blair"),
        ("mark twain", "samuel clemens"),
        ("lewis carroll", "charles dodgson"),
    ];
    
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    
    for (variant1, variant2) in known_variants.iter() {
        if (a_lower.contains(variant1) && b_lower.contains(variant2)) ||
           (a_lower.contains(variant2) && b_lower.contains(variant1)) {
            return true;
        }
    }
    false
}
pub async fn run_batch_example(pool: &SqlitePool) -> Result<(), RitmoErr> {
    // Processa batch di nomi
    process_people_batch(pool).await?;
    
    // Processa batch di tag
    process_tags_batch(pool).await?;
    
    // Carica i dati salvati
    load_saved_ml_data(pool).await?;
    
    println!("\n=== PROCESSAMENTO BATCH COMPLETATO ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_batch_processing() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        
        // Crea la tabella necessaria
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS ml_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data_type TEXT NOT NULL UNIQUE,
                data_json TEXT NOT NULL
            )
        "#)
        .execute(&pool)
        .await
        .unwrap();

        // Esegui il test
        run_batch_example(&pool).await.unwrap();
    }
}