use sqlx::query_as;
use crate::ml::people::parse_names::ParsedName;
use sqlx::SqlitePool;
use sqlx::query;
use crate::ml::people::record::PersonRecord;
use crate::errors::RitmoErr;
use crate::ml::utils::MLStringUtils;
use crate::ml::{entity_learner::MLEntityLearner, entity_persistence::load_ml_from_db};
use std::collections::HashMap;


/// Esempio di immissione di un nuovo nome nel sistema
pub async fn process_new_name_input(
    pool: &SqlitePool,
    input_name: &str,
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
    println!(
        "Caricati {} cluster e {} pattern ML",
        ml_data.clusters.len(),
        ml_data.learned_patterns.len()
    );

    // 4. Carica i nomi esistenti dal database
    let existing_records = load_existing_people(pool).await?;
    println!(
        "Caricati {} nomi esistenti dal database",
        existing_records.len()
    );

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

/// Controlla i pattern ML per possibili match
fn check_ml_patterns(new_record: &PersonRecord, ml_data: &MLEntityLearner) -> Vec<MLMatch> {
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
        let variant_similarity =
            strsim::jaro_winkler(&new_record.normalized_key, &pattern.variant_form);

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
    existing_records: &[PersonRecord],
) -> Vec<ExistingMatch> {
    let mut matches = Vec::new();

    for existing in existing_records {
        let similarity = strsim::jaro_winkler(&new_record.normalized_key, &existing.normalized_key);

        if similarity > 0.85 {
            matches.push(ExistingMatch {
                record_id: existing.id,
                matched_name: existing.original_input.clone(),
                similarity,
            });
        }

        // Controlla anche gli alias
        /*
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
        */
    }
    matches
}

/// Determina l'azione da intraprendere
fn determine_action(
    record: PersonRecord,
    ml_matches: Vec<MLMatch>,
    existing_matches: Vec<ExistingMatch>,
) -> ProcessingResult {
    // Trova il match esistente con similarità più alta
    let best_existing = existing_matches
        .iter()
        .max_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap());

    // Trova il match ML con confidence più alta
    let best_ml = ml_matches
        .iter()
        .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());

    let action = match (best_existing, best_ml) {
        // Match esatto nei nomi esistenti
        (Some(existing), _) if existing.similarity > 0.95 => {
            ProcessingAction::Duplicate(existing.record_id)
        }

        // Match ML molto alto
        (_, Some(ml)) if ml.confidence > 0.95 => ProcessingAction::RequiresVerification,

        // Match moderato - richiede verifica
        (Some(existing), _) if existing.similarity > 0.85 => ProcessingAction::RequiresVerification,

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

async fn insert_new_person(pool: &SqlitePool, record: &PersonRecord) -> Result<i64, RitmoErr> {
    // Gestione di middle_names: Se è vuoto, usa None; altrimenti, unisci e usa Some.
    let middle_names_str = if record.parsed_name.middle_names.is_empty() {
        None
    } else {
        Some(record.parsed_name.middle_names.join(", "))
    };

    // Esegui la query INSERT OR REPLACE
    //
    // Note:
    // 1. `id` è AUTOINCREMENT, quindi non dovremmo passarlo se vogliamo che il DB lo generi.
    //    Tuttavia, dato che usi `INSERT OR REPLACE`, stai esplicitamente provando a inserire
    //    o aggiornare un record con un ID specifico. Se `record.id` è 0 o nullo,
    //    l'INSERT OR REPLACE potrebbe comportarsi in modo inatteso per la generazione dell'ID.
    //    Assumo che tu voglia un comportamento in cui l'ID sia fornito da `record.id`
    //    e la riga venga aggiornata se l'ID esiste già.
    // 2. `created_at` e `updated_at` hanno DEFAULT `strftime('%s', 'now')` nel DB.
    //    Per `INSERT OR REPLACE`, se l'ID esiste, `created_at` non dovrebbe cambiare.
    //    `updated_at` sarà aggiornato dal trigger o dal DEFAULT della colonna.
    //    Quindi, è meglio Omettere `created_at` dalla query di INSERT OR REPLACE
    //    e lasciare che il DB lo imposti al primo insert, e lasciare che il trigger
    //    `update_ml_data_timestamp` (se pertinente a `people` - non lo è nel tuo schema fornito,
    //    ma l'ho menzionato) o il default di colonna gestiscano `updated_at`.
    //    In questo caso, siccome `created_at` e `updated_at` sono sempre `strftime('%s', 'now')`
    //    nell'INSERT, per `INSERT OR REPLACE` saranno sempre aggiornati all'ora corrente,
    //    anche `created_at` se la riga esiste già, il che potrebbe non essere il comportamento desiderato.
    //    Per `created_at`, se stai facendo un INSERT OR REPLACE e vuoi mantenere il `created_at`
    //    originale su un REPLACE, dovresti usare COALESCE(?, strftime('%s', 'now')) oppure
    //    leggere il valore esistente e passarlo, ma la soluzione più semplice è ometterlo
    //    o lasciare che il DB lo gestisca solo su un INSERT.
    //    Dato il tuo schema e l'uso di `INSERT OR REPLACE`, l'attuale approccio è accettabile
    //    se vuoi che `created_at` e `updated_at` siano *sempre* la data/ora dell'operazione.
    //    Se invece vuoi che `created_at` sia solo la data di creazione, dovresti usare `ON CONFLICT`
    //    o una logica `INSERT` separata da `UPDATE`.
    //    Per questa correzione, manterrò l'approccio originale per `created_at` e `updated_at`
    //    come li hai definiti nella query, assumendo che sia il comportamento desiderato
    //    per INSERT OR REPLACE.

    let query_result = query!(
        r#"
        INSERT OR REPLACE INTO people (
            id, name, given_name, surname, middle_names, title, suffix,
            display_name, normalized_key, confidence, verified, created_at, updated_at, source
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'), ?)
        "#, // Usare r#""# per stringhe raw multiline evita problemi di escape
        record.id,
        record.original_input, // Assumendo che 'name' in DB corrisponda a 'original_input' in PersonRecord
        record.parsed_name.given_name,
        record.parsed_name.surname,
        middle_names_str,
        record.parsed_name.title,
        record.parsed_name.suffix,
        record.parsed_name.display_name,
        record.normalized_key,
        record.confidence,
        record.verified,
        "biblioteca", // Non è necessario to_string() per una stringa letterale che sqlx converte in Text
    )
    .execute(pool)
    .await
    .map_err(|e| RitmoErr::DatabaseError(format!("Failed to save person record: {}", e)))?; // Preferisci un errore più specifico come DatabaseError

    // SQLx per `INSERT OR REPLACE` su SQLite restituisce `rows_affected()`.
    // Se `id` è generato automaticamente (AUTOINCREMENT) e non specificato,
    // `last_insert_rowid()` sarebbe più appropriato dopo un INSERT.
    // Ma con `INSERT OR REPLACE` e specificando l'ID, `rows_affected()`
    // è il modo corretto per verificare se un'operazione è avvenuta.
    //
    // `rows_affected()` sarà 1 se una nuova riga è stata inserita.
    // Sarà 0 o 1 (a seconda del driver SQLite e se i valori sono identici) se una riga è stata aggiornata
    // senza cambiamenti significativi.
    // In SQLite, per INSERT OR REPLACE:
    // - Se la riga viene inserita, rows_affected = 1.
    // - Se la riga viene aggiornata, rows_affected = 1 (sempre, anche se nessun valore cambia).
    // Quindi, se `rows_affected()` è 1, significa che l'operazione è andata a buon fine.
    // Il caso di `rows_affected() == 0` dovrebbe essere estremamente raro con INSERT OR REPLACE.
    // L'errore dovrebbe già essere gestito dal `.map_err()` precedente se la query fallisce.
    // Quindi, questo controllo `if result.rows_affected() == 0` potrebbe non essere necessario
    // o indicare un problema più profondo se mai si verificasse.
    // Per un `INSERT OR REPLACE` con ID specificato, se la query non fallisce, una riga
    // dovrebbe sempre essere "affected".

    // Restituisci l'ID del record. Se stai usando `INSERT OR REPLACE` e passi l'ID,
    // allora l'ID inserito/aggiornato è semplicemente `record.id`.
    // Se la colonna `id` fosse AUTOINCREMENT e non la passassi per nuovi inserimenti,
    // allora useresti `query_result.last_insert_rowid()` dopo un `INSERT` puro.
    // Dato l'uso di `INSERT OR REPLACE` con `record.id`, possiamo semplicemente restituire quello.
    // Tuttavia, il tipo di ritorno della funzione è `i64`. Se `record.id` è `Option<i64>` o simile,
    // dovresti gestirlo. Assumendo che `record.id` sia `i64`.
    Ok(record.id)
}

/// Esempio di utilizzo
pub async fn example_usage(pool: &SqlitePool) -> Result<(), RitmoErr> {
    // Esempi di input diversi
    let test_names = vec![
        "Stephen King",         // Nuovo nome
        "Steven King",          // Typo di Stephen King
        "S. King",              // Abbreviazione
        "Dr. Stephen King Jr.", // Con titolo e suffisso
        "King, Stephen",        // Formato diverso
        "Madonna",              // Nome singolo
        "J.K. Rowling",         // Nome completamente nuovo
    ];

    for name in test_names {
//        println!("\n" + "=".repeat(50).as_str());
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

async fn load_existing_people(pool: &SqlitePool) -> Result<Vec<PersonRecord>, RitmoErr> {
    // query_as! richiede una struct ausiliaria che possa essere mappata direttamente
    // dalle colonne del database. `PersonRecord` e `ParsedName` non lo sono,
    // a causa di:
    // - `parsed_name` essere una struct annidata.
    // - `middle_names` essere un `Vec<String>` (nel DB è `TEXT`).
    // - `given_name`, `surname`, `display_name` e `original_input` sono `String` non `Option<String>`
    //   mentre nel DB possono essere NULL.
    // - `verified` è `bool` (nel DB è `INTEGER`).

    let people_rows = query_as!(
        PersonRecordRow, // Questa struct verrà definita sotto
        r#"
        SELECT
            id,
            name AS original_input_db, -- Alias per evitare conflitti con il campo 'name' di PersonRecordRow
            given_name,
            surname,
            middle_names AS middle_names_db, -- Alias per distinguere dal Vec<String>
            title,
            suffix,
            display_name,
            normalized_key,
            confidence,
            verified AS verified_db, -- Alias per distinguere dal bool
            created_at,
            updated_at,
            source
        FROM
            people
        ORDER BY
            name
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| RitmoErr::DatabaseError(format!("Failed to load existing people: {}", e)))?;

    let mut person_records: Vec<PersonRecord> = Vec::with_capacity(people_rows.len());

    for row in people_rows {
        // Conversione da Option<String> a String per i campi non opzionali
        let given_name = row.given_name.unwrap_or_default();
        let surname = row.surname.unwrap_or_default();
        let display_name = row.display_name.unwrap_or_default();
        let original_input = row.original_input_db.unwrap_or_default();
        let normalized_key = row.normalized_key.unwrap_or_default(); // normalized_key è String, ma Option<String> nel DB

        // Conversione da INTEGER (DB) a bool
        let verified = row.verified_db.map(|v| v != 0).unwrap_or(false); // 0 = false, non-zero = true

        let parsed_name = ParsedName {
            given_name,
            surname,
            // Converti la stringa "middle_names" del DB (TEXT) in Vec<String>
            middle_names: row.middle_names_db
                            .filter(|s| !s.is_empty()) // Filtra stringhe vuote
                            .map(|s| s.split(", ").map(String::from).collect()) // Splitta per ", "
                            .unwrap_or_default(), // Se None, usa un Vec vuoto
            title: row.title,
            suffix: row.suffix,
            display_name,
        };

        let person_record = PersonRecord {
            id: row.id,
            original_input,
            parsed_name,
            normalized_key,
            confidence: row.confidence.expect("REASON"),
            verified, // Usa il valore bool convertito
            // created_at e updated_at non sono nei tuoi PersonRecord, quindi non li mappiamo qui.
            // Se li volessi, dovresti aggiungerli alla struct PersonRecord.
            // source non è in PersonRecord, quindi non lo mappiamo qui.
        };
        person_records.push(person_record);
    }

    Ok(person_records)
}

// Aggiungi questa struct nel tuo file models.rs o dove gestisci le tue struct dati.
// Assicurati di avere `sqlx = { version = "...", features = ["...", "macros"] }` nel Cargo.toml
// La macro `sqlx::FromRow` richiede che tutti i campi siano pubblici.
#[derive(sqlx::FromRow, Debug, Clone)]
struct PersonRecordRow {
    pub id: i64,
    pub original_input_db: Option<String>, // Colonna 'name' del DB
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names_db: Option<String>, // Colonna 'middle_names' del DB
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub display_name: Option<String>,
    pub normalized_key: Option<String>,
    pub confidence: Option<f64>,
    pub verified_db: Option<i64>, // Colonna 'verified' del DB (INTEGER)
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub source: Option<String>, // Colonna 'source' del DB
}
