// File: src/main.rs
// Punto di ingresso dell'applicazione per leggere nomi da un file e avviare il training ML.
use async_std::main; // Per la funzione main asincrona
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

// Importa il gestore nomi e i record specifici del progetto.
use crate::db::name_manager::{NameManager, PersonRecord, MatchResult}; // Importa MatchResult
// Importa l'enum degli errori specifici dell'applicazione.
use crate::errors::RitmoErr;

#[main] // Marca la funzione main come asincrona, necessaria per i metodi `async` di NameManager.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Applicazione per la gestione e il training dei nomi avviata.");

    // 1. Inizializzazione di NameManager.
    let mut name_manager = NameManager::new();

    // 2. Lettura dei nomi dal file 'names.txt'.
    let file_path = "names.txt";
    println!("Tentativo di leggere i nomi dal file: {}", file_path);

    let file = File::open(file_path)?; // Apre il file, gestisce errori.
    let reader = BufReader::new(file); // Crea un buffer per leggere le righe efficientemente.

    let mut current_id = 0; // Utilizzato per assegnare ID univoci ai record delle persone.
    let mut names_from_file: Vec<String> = Vec::new(); // Vettore per memorizzare i nomi letti dal file.

    // Itera su ogni riga del file.
    for line_res in reader.lines() {
        let line = line_res?; // Estrae la riga o propaga l'errore.
        if line.trim().is_empty() {
            continue; // Salta le righe vuote o che contengono solo spazi bianchi.
        }
        names_from_file.push(line.trim().to_string()); // Aggiunge il nome pulito al vettore.
    }

    if names_from_file.is_empty() {
        eprintln!("AVVISO: Nessun nome trovato nel file '{}'. Assicurati che il file esista e contenga nomi validi.", file_path);
        return Ok(()); // Termina se non ci sono nomi da processare.
    }
    println!("{} nomi letti con successo dal file.", names_from_file.len());

    // 3. Elaborazione e aggiunta dei record di persone al NameManager dal file iniziale.
    // Per ogni nome letto, crea un PersonRecord e lo aggiunge al NameManager.
    for name_input in names_from_file {
        current_id += 1; // Incrementa l'ID per ogni nuovo record.
        match name_manager.create_person_record(&name_input, current_id) {
            Ok(person_record) => {
                println!("  Creando record iniziale per: '{}' (ID: {})", person_record.original_input, person_record.id);
                name_manager.add_person_record(person_record)?;
            },
            Err(e) => {
                eprintln!("Errore nella creazione del record iniziale per '{}': {}", name_input, e);
            }
        }
    }
    println!("{} record di persone aggiunti al NameManager dall'input iniziale.", name_manager.all_person_records.len());

    // 4. Esecuzione del training del modello ML (con i dati iniziali).
    println!("\nAvvio del training del modello ML...");
    name_manager.train_ml_model()?; // Esegue il training del modello ML.

    // 5. Inserimento di nuovi nomi con controllo dei duplicati.
    println!("\nInizio inserimento di nuovi nomi con controllo duplicati...");
    let names_to_insert = vec![
        "Giuseppe Verdi".to_string(), // Probabilmente già presente se "Giuseppe" è nel file
        "Mario Rossi".to_string(),    // Probabilmente già presente se "Mario" è nel file
        "Luca Bianchini".to_string(), // Nome che non dovrebbe essere un duplicato esatto.
        "Luuka Bianchini".to_string(), // Una possibile variante/typo di "Luca Bianchini"
        "Francesco D'Agostino".to_string(), // Un altro nuovo nome
        "Fra D'Agostino".to_string(), // Un'abbreviazione
        "Giovanni Bacci".to_string(), // Nuovo
        "Giovani Baci".to_string(), // Variante fonetica/typo
    ];

    for new_name_input in names_to_insert {
        println!("\nProcessando nome: '{}'", new_name_input);

        // Prima di aggiungere, cerca match esistenti
        let match_result = name_manager.find_matches(&new_name_input);

        match match_result {
            MatchResult::ExactMatch(id) => {
                println!("  -> MATCH ESATTO trovato per '{}' con ID: {}. Non verrà aggiunto come nuovo record.", new_name_input, id);
            },
            MatchResult::HighConfidenceMatch(matches) => {
                println!("  -> MATCH AD ALTA CONFIDENZA trovato per '{}':", new_name_input);
                for m in matches {
                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
                }
                println!("  Non verrà aggiunto come nuovo record, potrebbe essere una variante/duplicato molto simile.");
                // Qui potresti decidere di aggiornare il record esistente con un alias
                // o fare altre azioni basate sull'alta confidenza.
                // Esempio (per scopi dimostrativi):
                // let base_name = name_manager.all_person_records.get(&matches[0].person_id).map(|p| p.normalized_key.clone());
                // if let Some(base) = base_name {
                //    name_manager.incremental_learning(vec![(base, new_name_input.clone(), 0.99)])?;
                // }
            },
            MatchResult::PossibleMatches(matches) => {
                println!("  -> POSSIBILI MATCH trovati per '{}':", new_name_input);
                for m in matches {
                    println!("    - ID: {}, Nome: '{}', Score: {:.2}, Tipo: {:?}", m.person_id, m.matched_name, m.similarity_score, m.match_type);
                }
                println!("  Considerando l'aggiunta come nuovo record, ma tieni d'occhio questi possibili match.");
                // In questo caso, puoi decidere di aggiungere il record ma magari con un flag di "da revisionare".
                add_new_record(&mut name_manager, &mut current_id, &new_name_input)?;
            },
            MatchResult::NoMatch => {
                println!("  -> NESSUN MATCH trovato per '{}'. Aggiungerò come nuovo record.", new_name_input);
                add_new_record(&mut name_manager, &mut current_id, &new_name_input)?;
            },
        }
    }

    println!("\nProcesso di inserimento completato.");
    println!("Numero finale di record nel gestore: {}", name_manager.all_person_records.len());
    println!("\nApplicazione terminata con successo.");

    // --- Esempio di unificazione manuale dei record ---
    // Questi ID sono fittizi e dovrebbero essere identificati dalla tua logica
    // o tramite revisione manuale dell'output del log.
    // Assumiamo che ID 1 e ID 2 siano Giuseppe Verdi e Giusepe Verdi che vogliamo unire.
    println!("\n--- Simulazione di unificazione manuale dei record ---");
    let initial_record_count = name_manager.all_person_records.len();
    if initial_record_count >= 2 { // Assicurati che ci siano almeno due record per la fusione.
        // ID esempio per dimostrazione. Nella realtà, li identificheresti dopo un'analisi.
        let keeper_id = 1; // ID del record da mantenere
        let duplicate_id = 2; // ID del record da unire e rimuovere

        // Verifica che gli ID esistano prima di tentare la fusione
        if name_manager.all_person_records.contains_key(&keeper_id) && name_manager.all_person_records.contains_key(&duplicate_id) {
            println!("Tentativo di unire record con ID {} nel record con ID {}", duplicate_id, keeper_id);
            match name_manager.merge_person_records(keeper_id, duplicate_id) {
                Ok(_) => {
                    println!("Unificazione completata con successo.");
                    println!("Numero finale di record dopo l'unificazione: {}", name_manager.all_person_records.len());
                },
                Err(e) => {
                    eprintln!("Errore durante l'unificazione: {}", e);
                }
            }
        } else {
            println!("Impossibile simulare l'unificazione: gli ID specificati non esistono nel gestore.");
        }
    } else {
        println!("Numero insufficiente di record per simulare l'unificazione.");
    }
    println!("--- Fine simulazione unificazione ---");

    Ok(())
}

// Funzione ausiliaria per aggiungere un nuovo record
fn add_new_record(
    name_manager: &mut NameManager,
    current_id: &mut i64,
    name_input: &str,
) -> Result<(), RitmoErr> {
    *current_id += 1; // Incrementa l'ID per il nuovo record
    match name_manager.create_person_record(name_input, *current_id) {
        Ok(new_record) => {
            println!("    Aggiungendo nuovo record in memoria per '{}' con ID: {}", new_record.parsed_name.display_name, new_record.id);
            name_manager.add_person_record(new_record)?;
        },
        Err(e) => {
            eprintln!("    Errore nella creazione del record per '{}': {}", name_input, e);
        }
    }
    Ok(())
}


// --- File: src/errors.rs ---
// Questo file definisce l'enum `RitmoErr` per la gestione centralizzata degli errori.
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Enum per la gestione degli errori specifici dell'applicazione.
/// Definisce vari tipi di errori che possono verificarsi all'interno del sistema Ritmo.
#[derive(Debug)]
pub enum RitmoErr {
    /// Errore generico di Input/Output, ad esempio durante la lettura o scrittura di file.
    IoError(String),
    /// Errore generico o non specificato. Utilizzato per casi non coperti da tipi più specifici.
    GenericError(String),
    /// Errore relativo alla logica di Machine Learning (es. training fallito, pattern non identificato).
    MlError(String),
    /// Errore di prestito del compilatore. Questo non dovrebbe apparire a runtime,
    /// è qui solo per mantenere la compatibilità con discussioni precedenti sulla risoluzione dei problemi di prestito.
    BorrowError(String),
    /// Errore durante l'esecuzione di una query SQL o l'interazione con il database.
    DatabaseQueryFailed(String),
    /// Errore durante la gestione di una transazione di database (es. avvio, commit, rollback).
    DatabaseTransactionError(String),
    /// Errore specifico durante la parsificazione di un nome, come input non valido.
    NameParsingError(String),
    /// Errore quando si tenta di unire record non validi (es. ID non trovati).
    MergeError(String), // NUOVO tipo di errore
    // Aggiungi qui altri tipi di errori se il tuo progetto ne introduce di nuovi.
}

// Implementazione del trait `Display` per `RitmoErr`
// Permette di stampare gli errori in un formato leggibile dall'utente.
impl Display for RitmoErr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            RitmoErr::IoError(msg) => write!(f, "Errore I/O: {}", msg),
            RitmoErr::GenericError(msg) => write!(f, "Errore generico: {}", msg),
            RitmoErr::MlError(msg) => write!(f, "Errore ML: {}", msg),
            RitmoErr::BorrowError(msg) => write!(f, "Errore di prestito: {}", msg),
            RitmoErr::DatabaseQueryFailed(msg) => write!(f, "Errore query database: {}", msg),
            RitmoErr::DatabaseTransactionError(msg) => write!(f, "Errore transazione database: {}", msg),
            RitmoErr::NameParsingError(msg) => write!(f, "Errore di parsificazione nome: {}", msg),
            RitmoErr::MergeError(msg) => write!(f, "Errore di unificazione: {}", msg), // NUOVO
        }
    }
}

// Implementazione del trait `Error` per `RitmoErr`
// È fondamentale per la gestione degli errori standard di Rust e per l'interoperabilità con `Box<dyn std::error::Error>`.
impl Error for RitmoErr {}

// Implementazione del trait `From` per convertire `std::io::Error` in `RitmoErr`.
// Consente di usare l'operatore `?` per errori I/O.
impl From<std::io::Error> for RitmoErr {
    fn from(err: std::io::Error) -> Self {
        RitmoErr::IoError(err.to_string())
    }
}

// Implementazione del trait `From` per convertire `sqlx::Error` in `RitmoErr`.
// Consente di usare l'operatore `?` per errori di database.
impl From<sqlx::Error> for RitmoErr {
    fn from(err: sqlx::Error) -> Self {
        RitmoErr::DatabaseQueryFailed(err.to_string())
    }
}


// --- File: src/db/mod.rs ---
// Questo file dichiara i moduli interni alla cartella `db`.
// `name_manager` contiene la logica principale di gestione dei nomi.
// `names_ml` contiene la logica specifica del machine learning.
pub mod name_manager;
pub mod names_ml;


// --- File: src/db/name_manager/mod.rs ---
// Contiene la definizione della struct `NameManager` e i suoi metodi principali per la gestione dei nomi.
// Include anche le struct e gli enum correlati come `PersonRecord`, `ParsedName`, ecc.

use sqlx::{Row, Transaction, Sqlite, SqlitePool, query}; // Per interazione con il database SQLite.
use human_name::Name; // Per parsificare i nomi in un formato strutturato.
use strsim::{jaro_winkler, levenshtein}; // Per calcolare la somiglianza tra stringhe.
use fuzzy_matcher::skim::SkimMatcherV2; // Per matching fuzzy.
use unicode_normalization::UnicodeNormalization; // Per normalizzazione Unicode dei nomi.
use serde::{Deserialize, Serialize}; // Per serializzazione e deserializzazione dei dati.
use std::collections::{HashMap, HashSet}; // Per strutture dati come hash map e hash set.
use std::error::Error; // Per il trait standard degli errori.
use rphonetic::{DoubleMetaphone, Encoder}; // Per la codifica fonetica Double Metaphone.

use crate::errors::RitmoErr; // Importa l'enum degli errori definiti globalmente.
use super::names_ml::{MLNameLearner, NameVariantPattern}; // Importa MLNameLearner dal sottomodulo `names_ml`.

/// Enum per la gestione degli errori interni specifici di `NameManager`.
/// Questi errori sono solitamente convertiti in `RitmoErr` prima di essere propagati.
#[derive(Debug)]
pub enum NameManagerErrorInternal {
    /// Errore durante il tentativo di parsificare una stringa in un formato di nome strutturato.
    NameParsingError(String),
}

// Implementazione del trait `Display` per `NameManagerErrorInternal`.
impl std::fmt::Display for NameManagerErrorInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NameManagerErrorInternal::NameParsingError(msg) => write!(f, "Errore di parsificazione nome interno: {}", msg),
        }
    }
}

// Implementazione del trait `Error` per `NameManagerErrorInternal`.
impl Error for NameManagerErrorInternal {}

// Implementazione del trait `From` per convertire `NameManagerErrorInternal` in `RitmoErr`.
// Permette una gestione degli errori più pulita e coerente.
impl From<NameManagerErrorInternal> for RitmoErr {
    fn from(err: NameManagerErrorInternal) -> Self {
        match err {
            NameManagerErrorInternal::NameParsingError(msg) => RitmoErr::NameParsingError(msg),
        }
    }
}

/// Struct per rappresentare un record completo di una persona, inclusi i dettagli del nome
/// e i metadati per il matching e il machine learning.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonRecord {
    pub id: i64, // ID univoco del record della persona.
    pub original_input: String, // Il nome come è stato originariamente inserito.
    pub parsed_name: ParsedName, // Il nome parsificato in componenti (dato, cognome, ecc.).
    pub normalized_key: String, // Versione normalizzata del nome per matching veloce.
    pub phonetic_key: String, // Chiave fonetica generata (es. Double Metaphone) per matching fonetico.
    pub confidence: f64, // Livello di confidenza associato a questo record.
    pub verified: bool, // Indica se il record è stato verificato manualmente.
    pub aliases: Vec<String>, // Elenco di alias o varianti note per questo nome.
}

/// Struct per rappresentare un nome parsificato in componenti strutturati.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)] // Deriva Default per creare istanze con valori predefiniti.
pub struct ParsedName {
    pub given_name: String, // Nome (es. Mario).
    pub surname: String, // Cognome (es. Rossi).
    pub middle_names: Vec<String>, // Nomi intermedi.
    pub title: Option<String>, // Titolo (es. Dr., Sig.).
    pub suffix: Option<String>, // Suffisso (es. Jr., III).
    pub display_name: String, // Il nome formattato per la visualizzazione (es. Mario Rossi).
}

/// Enum che rappresenta il risultato di un'operazione di matching di nomi.
#[derive(Debug)]
pub enum MatchResult {
    /// Corrispondenza esatta trovata con un ID persona.
    ExactMatch(i64),
    /// Corrispondenze con alta confidenza, include un elenco dei migliori match.
    HighConfidenceMatch(Vec<NameMatch>),
    /// Possibili corrispondenze, include un elenco di match meno certi.
    PossibleMatches(Vec<NameMatch>),
    /// Nessuna corrispondenza trovata.
    NoMatch,
}

/// Struct che rappresenta un singolo match trovato per un nome.
#[derive(Debug, Clone)]
pub struct NameMatch {
    pub person_id: i64, // ID della persona corrispondente.
    pub matched_name: String, // Il nome che ha corrisposto nel database.
    pub similarity_score: f64, // Punteggio di somiglianza tra 0 e 1.
    pub match_type: MatchType, // Il tipo di corrispondenza (esatta, fonetica, alias, ecc.).
    pub confidence: f64, // Livello di confidenza del match.
}

/// Enum che definisce i vari tipi di corrispondenza che possono essere trovati.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    Exact,             // Mario Rossi vs Mario Rossi
    NameOrder,         // Mario Rossi vs Rossi Mario
    Phonetic,          // Asimov vs Azimov
    Abbreviated,       // J.R.R. Tolkien vs John Ronald Reuel Tolkien
    Typo,              // Asimof vs Asimov (generico)
    Alias,             // Bob vs Robert
    PhoneticSimilar,   // NUOVO: matching fonetico basato su similarità fonetica.
    TypoMinor,         // NUOVO: errore di battitura minore (es. un singolo carattere).
    TypoMajor,         // NUOVO: errori di battitura multipli o più significativi.
    Learned,           // NUOVO: variante appresa dal modello di machine learning.
}

/// La struct `NameManager` gestisce l'indicizzazione, la normalizzazione e il matching dei nomi.
/// Integra la logica di machine learning per migliorare il riconoscimento delle varianti.
pub struct NameManager {
    fuzzy_matcher: SkimMatcherV2, // Matcher per ricerca fuzzy.
    double_metaphone: DoubleMetaphone, // Encoder fonetico.
    common_abbreviations: HashMap<String, Vec<String>>, // Abbreviazioni comuni predefinite.
    similarity_threshold: f64, // Soglia di somiglianza per i match.
    typo_threshold: f64, // Soglia specifica per il riconoscimento dei typo.
    all_person_records: HashMap<i64, PersonRecord>, // Tutti i record di persone indicizzati per ID.
    normalized_key_index: HashMap<String, HashSet<i64>>, // Indice dei nomi normalizzati.
    phonetic_key_index: HashMap<String, HashSet<i64>>, // Indice delle chiavi fonetiche.
    name_variants: HashMap<String, Vec<String>>, // Varianti di nomi conosciute e predefinite.
    ml_learner: MLNameLearner, // Istanza del learner di machine learning.
}

impl NameManager {
    /// Costruttore per `NameManager`. Inizializza tutti i campi con valori predefiniti
    /// e popola le mappe di abbreviazioni e varianti comuni.
    pub fn new() -> Self {
        let mut common_abbreviations = HashMap::new();
        common_abbreviations.insert("giuseppe".to_string(), vec!["peppe".to_string(), "beppe".to_string()]);
        common_abbreviations.insert("giovanni".to_string(), vec!["gianni".to_string(), "gian".to_string()]);
        common_abbreviations.insert("francesco".to_string(), vec!["franco".to_string(), "checco".to_string()]);

        let mut name_variants = HashMap::new();
        name_variants.insert("anton".to_string(), vec!["antonio".to_string(), "antony".to_string()]);
        name_variants.insert("pavlovic".to_string(), vec!["pavlociv".to_string(), "pavlovič".to_string()]);
        name_variants.insert("cechov".to_string(), vec!["chekhov".to_string(), "čechov".to_string(), "tchekhov".to_string()]);
        name_variants.insert("franc".to_string(), vec!["frank".to_string(), "franck".to_string(), "francesco".to_string()]);

        Self {
            fuzzy_matcher: SkimMatcherV2::default(),
            double_metaphone: DoubleMetaphone::default(),
            common_abbreviations,
            similarity_threshold: 0.75, // Soglia di somiglianza ridotta da 0.8
            typo_threshold: 0.85, // Soglia specifica per i typo
            all_person_records: HashMap::new(),
            normalized_key_index: HashMap::new(),
            phonetic_key_index: HashMap::new(),
            name_variants,
            ml_learner: MLNameLearner::new(), // Inizializza il learner ML
        }
    }

    /// Normalizza una stringa per il matching, convertendola in minuscolo,
    /// rimuovendo accenti e caratteri speciali, e riducendo spazi multipli.
    pub fn normalize_string(&self, text: &str) -> String {
        let normalized = text
            .nfc() // Normalizzazione Unicode (Forma C di normalizzazione)
            .collect::<String>()
            .to_lowercase() // Converte in minuscolo
            .chars()
            .map(|c| match c { // Mappa i caratteri per rimuovere accenti e sostituire speciali
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
                // Mantiene caratteri alfabetici e spazi, sostituisce altri con uno spazio
                c if c.is_alphabetic() || c.is_whitespace() => c,
                _ => ' ',
            })
            .collect::<String>();

        // Rimuove spazi multipli e taglia gli spazi iniziali/finali
        normalized.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// Genera una chiave fonetica per una data stringa utilizzando Double Metaphone.
    /// Ogni parte del nome viene codificata separatamente e le chiavi primarie sono unite.
    pub fn generate_phonetic_key(&self, text: &str) -> String {
        let normalized = self.normalize_string(text);
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        let mut phonetic_parts = Vec::new();

        for part in parts {
            // `double_metaphone.encode` restituisce una tupla (primary_key, secondary_key_option)
            let (primary, _) = self.double_metaphone.encode(part);
            phonetic_parts.push(primary);
        }

        phonetic_parts.join(" ") // Unisce le chiavi fonetiche di ciascuna parte del nome.
    }

    /// Calcola la distanza di Levenshtein normalizzata tra due stringhe.
    /// Il risultato è un valore tra 0.0 e 1.0, dove 1.0 indica un match esatto.
    pub fn normalized_levenshtein_distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            return 1.0; // Se entrambe le stringhe sono vuote, sono un match perfetto.
        }
        // 1.0 - (distanza Levenshtein / lunghezza massima)
        1.0 - (levenshtein(s1, s2) as f64 / max_len)
    }

    /// Verifica se due nomi sono varianti conosciute e predefinite nella mappa `name_variants`.
    pub fn are_known_variants(&self, name1: &str, name2: &str) -> bool {
        let norm1 = self.normalize_string(name1);
        let norm2 = self.normalize_string(name2);

        // Controlla se norm1 è una base e norm2 è una delle sue varianti
        if let Some(variants) = self.name_variants.get(&norm1) {
            if variants.contains(&norm2) {
                return true;
            }
        }

        // Controlla il contrario (se norm2 è una base e norm1 è una delle sue varianti)
        if let Some(variants) = self.name_variants.get(&norm2) {
            if variants.contains(&norm1) {
                return true;
            }
        }

        false
    }

    /// Aggiunge un nuovo `PersonRecord` al gestore dei nomi, aggiornando gli indici
    /// per la chiave normalizzata e la chiave fonetica.
    pub fn add_person_record(&mut self, record: PersonRecord) -> Result<(), RitmoErr> {
        let id = record.id;
        let normalized_key = record.normalized_key.clone();
        let phonetic_key = record.phonetic_key.clone();

        self.all_person_records.insert(id, record.clone()); // Inserisce il record nella mappa principale.

        // Aggiunge l'ID all'indice per la chiave normalizzata.
        self.normalized_key_index.entry(normalized_key)
            .or_default()
            .insert(id);

        // Aggiunge l'ID all'indice per la chiave fonetica.
        self.phonetic_key_index.entry(phonetic_key)
            .or_default()
            .insert(id);

        // Indicizza anche gli alias del record.
        for alias in &record.aliases {
            let normalized_alias = self.normalize_string(alias);
            let phonetic_alias = self.generate_phonetic_key(alias);

            self.normalized_key_index.entry(normalized_alias)
                .or_default()
                .insert(id);

            self.phonetic_key_index.entry(phonetic_alias)
                .or_default()
                .insert(id);
        }

        Ok(())
    }

    /// Trova le corrispondenze per un nome di input, utilizzando diverse strategie
    /// di matching (esatto, ML, fonetico, invertito, alias).
    pub fn find_matches(&self, input_name: &str) -> MatchResult {
        // Parsifica il nome di input. Se fallisce, non ci sono match.
        let parsed_input_res = self.parse_name(input_name);
        if parsed_input_res.is_err() {
            return MatchResult::NoMatch;
        }

        let parsed_input = parsed_input_res.unwrap();
        let normalized_input = self.normalize_parsed_name_for_matching(&parsed_input);
        let phonetic_input = self.generate_phonetic_key(&normalized_input);

        let mut candidate_ids: HashSet<i64> = HashSet::new();

        // Cerca match esatti negli indici normalizzati e fonetici.
        if let Some(ids) = self.normalized_key_index.get(&normalized_input) {
            candidate_ids.extend(ids);
        }

        if let Some(ids) = self.phonetic_key_index.get(&phonetic_input) {
            candidate_ids.extend(ids);
        }

        // Cerca candidati usando la somiglianza di Levenshtein per i typo.
        for (normalized_key, ids) in &self.normalized_key_index {
            let levenshtein_sim = self.normalized_levenshtein_distance(&normalized_input, normalized_key);
            if levenshtein_sim >= self.typo_threshold {
                candidate_ids.extend(ids);
            }
        }

        if candidate_ids.is_empty() {
            return MatchResult::NoMatch; // Nessun candidato trovato, nessun match.
        }

        let mut matches = Vec::new();

        // Valuta ogni candidato trovato.
        for &person_id in &candidate_ids {
            if let Some(person) = self.all_person_records.get(&person_id) {
                let mut best_match: Option<NameMatch> = None;
                let mut best_score = 0.0;

                // 1. Match esatto o per typo (diretto con Jaro-Winkler).
                let direct_score = jaro_winkler(&normalized_input, &person.normalized_key);
                if direct_score > best_score {
                    best_score = direct_score;
                    let match_type = if direct_score >= 0.99 {
                        MatchType::Exact // Match quasi perfetto.
                    } else if direct_score >= self.typo_threshold {
                        // Distingui tra typo minori e maggiori basandosi sulla distanza Levenshtein.
                        let levenshtein_sim = self.normalized_levenshtein_distance(&normalized_input, &person.normalized_key);
                        if levenshtein_sim >= 0.9 {
                            MatchType::TypoMinor
                        } else {
                            MatchType::TypoMajor
                        }
                    } else {
                        MatchType::Typo // Typo generico se sotto la soglia specifica.
                    };

                    best_match = Some(NameMatch {
                        person_id: person.id,
                        matched_name: person.parsed_name.display_name.clone(),
                        similarity_score: direct_score,
                        match_type,
                        confidence: direct_score * person.confidence,
                    });
                }

                // 2. Verifica varianti apprese dal ML.
                if best_score < 1.0 { // Se non è già un match esatto.
                    if let Some(learned_variant) = self.ml_learner.find_learned_variant(&normalized_input, &person.normalized_key) {
                        best_score = learned_variant.confidence.max(0.88); // Imposta un punteggio minimo per le varianti apprese.
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: learned_variant.confidence,
                            match_type: MatchType::Learned,
                            confidence: learned_variant.confidence * person.confidence,
                        });
                    }
                }

                // 3. Verifica varianti conosciute (predefined).
                if best_score < 1.0 { // Se non è già un match migliore.
                    if self.are_known_variants(&normalized_input, &person.normalized_key) {
                        best_score = 0.95; // Alta confidenza per varianti conosciute.
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: 0.95,
                            match_type: MatchType::Alias,
                            confidence: 0.95 * person.confidence,
                        });
                    }
                }

                // 4. Match fonetico.
                if best_score < 1.0 { // Se non è già un match migliore.
                    let phonetic_score = jaro_winkler(&phonetic_input, &person.phonetic_key);
                    if phonetic_score >= 0.8 && phonetic_score > best_score * 0.9 { // Punteggio fonetico significativo.
                        best_score = phonetic_score * 0.9; // Penalizza leggermente il match fonetico.
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: phonetic_score,
                            match_type: MatchType::PhoneticSimilar,
                            confidence: phonetic_score * person.confidence * 0.85,
                        });
                    }
                }

                // 5. Match con nomi invertiti (es. "Rossi Mario" per "Mario Rossi").
                if best_score < 1.0 { // Se non è già un match migliore.
                    let person_swapped_parsed_name = ParsedName {
                        given_name: person.parsed_name.surname.clone(),
                        surname: person.parsed_name.given_name.clone(),
                        middle_names: person.parsed_name.middle_names.clone(),
                        title: person.parsed_name.title.clone(),
                        suffix: person.parsed_name.suffix.clone(),
                        display_name: format!("{} {}", person.parsed_name.surname, person.parsed_name.given_name),
                    };
                    let person_swapped_normalized_key = self.normalize_parsed_name_for_matching(&person_swapped_parsed_name);
                    let swap_score = jaro_winkler(&normalized_input, &person_swapped_normalized_key);

                    if swap_score > best_score && swap_score >= self.similarity_threshold {
                        best_score = swap_score;
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: swap_score,
                            match_type: MatchType::NameOrder,
                            confidence: swap_score * person.confidence,
                        });
                    }
                }

                // 6. Match con alias memorizzati per la persona.
                if best_score < 1.0 { // Se non è già un match migliore.
                    for alias in &person.aliases {
                        let alias_score = jaro_winkler(&normalized_input, &self.normalize_string(alias));
                        if alias_score > best_score {
                            best_score = alias_score;
                            best_match = Some(NameMatch {
                                person_id: person.id,
                                matched_name: alias.clone(),
                                similarity_score: alias_score,
                                match_type: MatchType::Alias,
                                confidence: alias_score * person.confidence * 0.9,
                            });
                        }
                    }
                }

                // Se un match è stato trovato e supera la soglia di somiglianza, lo aggiunge alla lista.
                if let Some(m) = best_match {
                    if m.similarity_score >= self.similarity_threshold {
                        matches.push(m);
                    }
                }
            }
        }

        // Ordina i match dal più confidente al meno confidente.
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        // Restituisce un `ExactMatch` se c'è una corrispondenza quasi perfetta.
        if let Some(perfect_match) = matches.iter().find(|m| m.similarity_score >= 0.99) {
            return MatchResult::ExactMatch(perfect_match.person_id);
        }

        // Restituisce il tipo di match basato sul numero e sulla confidenza dei risultati.
        match matches.len() {
            0 => MatchResult::NoMatch, // Nessun match significativo.
            _ => {
                let top_match = &matches[0];
                if top_match.confidence > 0.9 {
                    // Match ad alta confidenza (primi 3).
                    MatchResult::HighConfidenceMatch(matches.into_iter().take(3).collect())
                } else if top_match.confidence > 0.75 {
                    // Possibili match (primi 5).
                    MatchResult::PossibleMatches(matches.into_iter().take(5).collect())
                } else {
                    MatchResult::NoMatch // Match non sufficientemente confidente.
                }
            }
        }
    }

    /// Crea un nuovo `PersonRecord` da un input di stringa, parsificando il nome
    /// e generando le chiavi normalizzata e fonetica.
    pub fn create_person_record(&self, input: &str, id: i64) -> Result<PersonRecord, RitmoErr> {
        let parsed_name = self.parse_name(input)?; // Parsifica il nome di input.
        let normalized_key = self.normalize_parsed_name_for_matching(&parsed_name); // Genera la chiave normalizzata.
        let phonetic_key = self.generate_phonetic_key(&normalized_key); // Genera la chiave fonetica.

        Ok(PersonRecord {
            id,
            original_input: input.to_string(),
            parsed_name,
            normalized_key,
            phonetic_key, // La chiave fonetica è ora valorizzata.
            confidence: 1.0, // Confidenza iniziale alta per i nuovi record.
            verified: false, // Non verificato di default.
            aliases: Vec::new(), // Nessun alias iniziale.
        })
    }

    /// Aggiunge una nuova coppia base-variante alla mappa delle varianti conosciute.
    /// Aggiunge la variante in entrambe le direzioni per facilitare la ricerca.
    pub fn add_name_variant(&mut self, base_name: &str, variant: &str) {
        let base_normalized = self.normalize_string(base_name);
        let variant_normalized = self.normalize_string(variant);

        // Aggiunge la variante associata alla forma base.
        self.name_variants
            .entry(base_normalized.clone())
            .or_default()
            .push(variant_normalized.clone());

        // Aggiunge la forma base associata alla variante (relazione bidirezionale).
        self.name_variants
            .entry(variant_normalized)
            .or_default()
            .push(base_normalized);
    }

    /// Parsifica una stringa di nome in una `ParsedName` strutturata utilizzando la libreria `human-name`.
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
        let display_name = parsed.display_first_last(); // Ottiene il nome formattato per la visualizzazione.

        Ok(ParsedName {
            given_name,
            surname,
            middle_names,
            title,
            suffix,
            display_name: display_name.to_string(),
        })
    }

    /// Normalizza un `ParsedName` per il matching, combinando i suoi componenti e normalizzandoli.
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

        let combined_name = full_name_parts.join(" "); // Unisce le parti del nome con spazi.
        self.normalize_string(&combined_name) // Normalizza la stringa combinata.
    }

    // Metodi per il machine learning.
    /// Avvia il processo di apprendimento automatico sui dati esistenti in `NameManager`.
    pub fn train_ml_model(&mut self) -> Result<(), RitmoErr> {
        println!("  Inizio training del modello ML per il riconoscimento delle varianti di nomi...");

        // 1. Raccoglie tutti i nomi normalizzati esistenti per l'analisi ML.
        let all_names: Vec<String> = self.all_person_records
            .values()
            .map(|record| record.normalized_key.clone())
            .collect();

        // 2. Crea cluster di nomi simili utilizzando il `ml_learner`.
        self.ml_learner.create_name_clusters(&all_names, &self.double_metaphone)?;

        // 3. Identifica pattern nelle varianti. Questo metodo sulla MLNameLearner non prende argomenti diretti,
        // ma opera sui dati già presenti nel learner o sui risultati di `create_name_clusters`.
        self.ml_learner.identify_variant_patterns()?;

        // 4. Applica le varianti apprese al sistema di matching di `NameManager`.
        self.apply_learned_variants()?;

        println!("  Training completato. {} pattern appresi, {} cluster creati.",
                 self.ml_learner.learned_patterns.len(),
                 self.ml_learner.name_clusters.len());

        Ok(())
    }

    /// Applica le varianti di nomi apprese dal modello ML alla mappa `name_variants`
    /// di `NameManager`, rendendole utilizzabili per il matching.
    fn apply_learned_variants(&mut self) -> Result<(), RitmoErr> {
        println!("  Applicazione delle varianti apprese (logica ML effettiva)...");
        // Filtra i pattern appresi che superano la soglia di confidenza minima del learner.
        let high_confidence_patterns: Vec<_> = self.ml_learner.learned_patterns
            .iter()
            .filter(|pattern| pattern.confidence >= self.ml_learner.minimum_confidence)
            .cloned() // Clona i pattern per prenderne possesso.
            .collect();

        // Aggiunge ogni pattern appreso come una variante conosciuta.
        for pattern in high_confidence_patterns {
            self.add_name_variant(&pattern.base_form, &pattern.variant_form);
        }

        Ok(())
    }

    /// Implementa l'apprendimento incrementale per il modello ML, aggiungendo
    /// varianti osservate e ritrainando periodicamente.
    pub fn incremental_learning(&mut self, observed_matches: Vec<(String, String, f64)>) -> Result<(), RitmoErr> {
        for (name1, name2, confidence) in observed_matches {
            if confidence >= 0.8 { // Solo match con alta confidenza contribuiscono all'apprendimento.
                self.ml_learner.add_observed_variant(&name1, &name2, confidence)?;
            }
        }

        // Ritraina il modello ML ogni 100 nuovi pattern osservati (per efficienza).
        if self.ml_learner.pattern_frequency.len() % 100 == 0 {
            self.train_ml_model()?;
        }

        Ok(())
    }

    // Metodi del database. Questi richiedono una configurazione di `sqlx` e un database SQLite.
    // Sono inclusi qui come da tuo input, ma non vengono chiamati direttamente da `main.rs` nell'esempio corrente.

    /// Carica i record di persone da un database SQLite nel `NameManager`.
    pub async fn load_names_from_db(&mut self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, given_name, surname, middle_names, title, suffix, display_name, normalized_key, confidence
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
            let confidence: f64 = row.try_get("confidence")?;
            let aliases: Vec<String> = Vec::new(); // Gli alias dovrebbero essere caricati separatamente o da un campo nel DB.

            let parsed_name = ParsedName {
                given_name,
                surname,
                middle_names,
                title,
                suffix,
                display_name,
            };

            // Genera la chiave fonetica al momento del caricamento.
            // Se il DB dovesse includere un campo `phonetic_key`, potresti leggerlo da lì.
            let phonetic_key = self.generate_phonetic_key(&normalized_key);

            let person_record = PersonRecord {
                id,
                original_input,
                parsed_name,
                normalized_key,
                phonetic_key, // La chiave fonetica è ora valorizzata.
                confidence,
                verified: true, // Presunto verificato se viene dal DB.
                aliases,
            };
            self.add_person_record(person_record)?;
        }
        Ok(())
    }

    /// Salva un singolo `PersonRecord` nel database all'interno di una transazione.
    pub async fn save_single_person_record_in_tx(
        &self,
        transaction: &mut Transaction<'_, Sqlite>, // Accetta una transazione mutabile.
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
        .bind(&record.phonetic_key) // Ora la chiave fonetica viene salvata.
        .bind(record.confidence)
        .bind(record.verified)
        .bind("biblioteca") // Il sorgente è hardcoded, potresti volerlo rendere configurabile.
        .execute(&mut **transaction) // Esegue la query all'interno della transazione.
        .await
        .map_err(|e| RitmoErr::DatabaseTransactionError(format!("Errore nel salvare il record persona nel DB durante la transazione: {}", e)))?;
        if result.rows_affected() == 0 {
            return Err(RitmoErr::DatabaseTransactionError(format!("Nessuna riga modificata per ID {} durante il salvataggio del record.", record.id)));
        }
        Ok(())
    }

    /// Salva un vettore di `PersonRecord` nel database all'interno di una singola transazione.
    pub async fn save_person_records_to_db(
        &self,
        pool: &SqlitePool,
        records: &Vec<PersonRecord>, // Accetta un riferimento a un vettore di record.
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
        Ok(())
    }

    /// Processa un elenco di nuovi nomi, trovando match esistenti o aggiungendo nuovi record
    /// e salvandoli nel database.
    pub async fn process_and_add_new_names(&mut self, pool: &SqlitePool, names_to_process: Vec<String>) -> Result<(), RitmoErr> {
        let mut new_person_records_to_add: Vec<PersonRecord> = Vec::new();
        // Trova l'ID massimo corrente per assegnare nuovi ID univoci.
        let mut current_max_id: i64 = self.all_person_records.keys().max().copied().unwrap_or(0);
        println!("Inizio elaborazione di {} nomi per l'aggiunta al DB...", names_to_process.len());

        for name_input in names_to_process {
            let match_result = self.find_matches(&name_input);
            let should_add_new = match match_result {
                MatchResult::ExactMatch(_) => {
                    // Se c'è un match esatto, non aggiungere un nuovo record.
                    false
                },
                MatchResult::HighConfidenceMatch(matches) => {
                    // Se c'è un match ad alta confidenza (>= 0.95), non aggiungere.
                    if matches[0].confidence >= 0.95 {
                        false
                    } else {
                        true // Altrimenti, potresti voler aggiungere una variante.
                    }
                },
                MatchResult::PossibleMatches(_) | MatchResult::NoMatch => {
                    // Se ci sono solo possibili match o nessun match, aggiungi un nuovo record.
                    true
                },
            };
            if should_add_new {
                current_max_id += 1; // Incrementa l'ID per il nuovo record.
                match self.create_person_record(&name_input, current_max_id) {
                    Ok(new_record) => {
                        println!("  Aggiunto nuovo record in memoria per '{}' con ID: {}", new_record.parsed_name.display_name, new_record.id);
                        new_person_records_to_add.push(new_record);
                    },
                    Err(e) => {
                        eprintln!("Errore nella creazione del record per '{}': {}", name_input, e);
                    }
                }
            }
        }
        if !new_person_records_to_add.is_empty() {
            println!("Salvataggio di {} nuovi record nel database...", new_person_records_to_add.len());
            // Salva i nuovi record nel database.
            // Nota: se la feature `db_support` non è abilitata in Cargo.toml,
            // questa riga causerà un errore di compilazione.
            // Dovrai scommentare e configurare sqlx per usarla.
            // self.save_person_records_to_db(pool, &new_person_records_to_add).await?;
            println!("Salvataggio simulato completato nel database (richiede feature 'db_support').");
            // Aggiungi i record anche al gestore in memoria dopo il salvataggio (simulato) nel DB.
            for record in new_person_records_to_add {
                self.add_person_record(record)?;
            }
        } else {
            println!("Nessun nuovo nome da aggiungere al database.");
        }
        Ok(())
    }

    /// Unisce due record di persone, collassando il `duplicate_id` nel `keeper_id`.
    ///
    /// Tutte le informazioni significative (input originale, alias, chiavi di indice)
    /// dal record duplicato vengono trasferite al record principale. Il record duplicato
    /// viene poi rimosso dal gestore.
    ///
    /// # Arguments
    /// * `keeper_id` - L'ID del record della persona da mantenere.
    /// * `duplicate_id` - L'ID del record della persona da unire e rimuovere.
    ///
    /// # Errors
    /// Restituisce `RitmoErr::MergeError` se uno o entrambi gli ID non vengono trovati,
    /// o se `keeper_id` e `duplicate_id` sono gli stessi.
    pub fn merge_person_records(&mut self, keeper_id: i64, duplicate_id: i64) -> Result<(), RitmoErr> {
        if keeper_id == duplicate_id {
            return Err(RitmoErr::MergeError("Impossibile unire un record con se stesso.".to_string()));
        }

        // Rimuovi il record duplicato per poter mutare keeper senza problemi di prestito.
        let duplicate_record = self.all_person_records.remove(&duplicate_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record duplicato con ID {} non trovato.", duplicate_id)))?;

        // Ottieni un riferimento mutabile al record principale.
        let keeper_record = self.all_person_records.get_mut(&keeper_id)
            .ok_or_else(|| RitmoErr::MergeError(format!("Record principale con ID {} non trovato.", keeper_id)))?;

        println!("  Unendo '{}' (ID: {}) in '{}' (ID: {})...",
                 duplicate_record.parsed_name.display_name, duplicate_record.id,
                 keeper_record.parsed_name.display_name, keeper_record.id);

        // Trasferisci l'input originale del duplicato come alias al keeper.
        if !keeper_record.aliases.contains(&duplicate_record.original_input) &&
           keeper_record.original_input != duplicate_record.original_input {
            keeper_record.aliases.push(duplicate_record.original_input.clone());
        }

        // Trasferisci tutti gli alias del duplicato al keeper.
        for alias in duplicate_record.aliases {
            if !keeper_record.aliases.contains(&alias) &&
               keeper_record.original_input != alias { // Evita di aggiungere l'original input del keeper come alias
                keeper_record.aliases.push(alias);
            }
        }

        // Aggiorna gli indici per puntare dal duplicato al keeper.
        // Itera su tutti gli indici e aggiorna l'ID del duplicato con l'ID del keeper.

        // Indice della chiave normalizzata
        let duplicate_normalized_key = duplicate_record.normalized_key.clone();
        if let Some(ids) = self.normalized_key_index.get_mut(&duplicate_normalized_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            // Se il set di ID per questa chiave normalizzata diventa vuoto, rimuovi la chiave.
            if ids.is_empty() {
                self.normalized_key_index.remove(&duplicate_normalized_key);
            }
        }

        // Indice della chiave fonetica
        let duplicate_phonetic_key = duplicate_record.phonetic_key.clone();
        if let Some(ids) = self.phonetic_key_index.get_mut(&duplicate_phonetic_key) {
            ids.remove(&duplicate_id);
            ids.insert(keeper_id);
            // Se il set di ID per questa chiave fonetica diventa vuoto, rimuovi la chiave.
            if ids.is_empty() {
                self.phonetic_key_index.remove(&duplicate_phonetic_key);
            }
        }

        // Qui potresti anche voler aggiornare il database se lo stai usando.
        // Esempio (richiede `sqlx` e `pool`):
        /*
        // Assicurati che il record keeper sia salvato con gli alias aggiornati.
        // Assicurati che il record duplicato sia eliminato.
        // Questo richiederebbe di passare un riferimento al `SqlitePool` o una transazione.
        // Ad esempio:
        // let mut tx = pool.begin().await?;
        // self.save_single_person_record_in_tx(&mut tx, keeper_record).await?; // Aggiorna il keeper
        // sqlx::query!("DELETE FROM people WHERE id = ?", duplicate_id)
        //     .execute(&mut *tx).await?; // Elimina il duplicato
        // tx.commit().await?;
        */

        Ok(())
    }
}


// --- File: src/db/names_ml.rs ---
// Contiene la logica specifica del machine learning per l'apprendimento delle varianti di nomi.

use std::collections::HashMap;
use serde::{Deserialize, Serialize}; // Necessario per Serializzazione/Deserializzazione.
use crate::errors::RitmoErr; // Importa l'enum degli errori.
use crate::db::name_manager::DoubleMetaphone; // Importa `DoubleMetaphone` dal modulo `name_manager`.

/// Struttura che rappresenta un pattern di variante di nome appreso dal modello ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameVariantPattern {
    pub base_form: String, // La forma base del nome (es. "giovanni").
    pub variant_form: String, // La forma variante del nome (es. "gianni").
    pub pattern_type: VariantPatternType, // Il tipo di pattern di variante (es. Abbreviation).
    pub confidence: f64, // Livello di confidenza che questo sia un pattern valido.
    pub frequency: usize, // Quante volte questo pattern è stato osservato.
    pub phonetic_similarity: f64, // Similarità fonetica tra base e variante.
    pub edit_distance: usize, // Distanza di edit (es. Levenshtein) tra base e variante.
}

/// Enum che definisce i vari tipi di pattern di variante di nome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum VariantPatternType {
    Suffix,          // Antonio → Anton
    Prefix,          // Giuseppe → Beppe
    Phonetic,        // Cechov → Chekhov
    Transliteration, // Павлович → Pavlovic
    Abbreviation,    // Francesco → Franco
    Compound,        // Jean-Pierre → Gianpiero
}

/// Struttura che rappresenta un cluster di nomi simili, raggruppati dal modello ML.
#[derive(Debug, Clone)]
pub struct NameCluster {
    pub cluster_id: usize, // ID univoco del cluster.
    pub members: Vec<String>, // Elenco dei nomi che appartengono a questo cluster.
    pub centroid: String, // Il nome rappresentativo (centroide) del cluster.
    pub phonetic_signature: String, // La firma fonetica del cluster.
    pub confidence: f64, // Livello di confidenza del cluster.
}

/// La struct `MLNameLearner` incapsula la logica di apprendimento automatico
/// per identificare e gestire i pattern di varianti di nomi.
#[derive(Debug, Default)]
pub struct MLNameLearner {
    pub learned_patterns: Vec<NameVariantPattern>, // Pattern di varianti di nomi appresi.
    pub name_clusters: HashMap<String, NameCluster>, // Cluster di nomi simili.
    pub pattern_frequency: HashMap<String, usize>, // Frequenza di osservazione dei pattern, usata per l'apprendimento incrementale.
    pub minimum_confidence: f64, // Soglia di confidenza minima per considerare un pattern appreso valido.
}

impl MLNameLearner {
    /// Costruttore per `MLNameLearner`. Inizializza i campi con valori predefiniti.
    pub fn new() -> Self {
        Self {
            minimum_confidence: 0.88, // Imposta una soglia predefinita.
            ..Default::default() // Utilizza l'implementazione `Default` per gli altri campi.
        }
    }

    /// Mock: Crea cluster di nomi simili basandosi sui nomi forniti.
    /// (Dovrai implementare la logica reale qui per il raggruppamento dei nomi).
    pub fn create_name_clusters(&mut self, _names: &[String], _double_metaphone: &DoubleMetaphone) -> Result<(), RitmoErr> {
        println!("    Creazione dei cluster di nomi (logica fittizia in MLNameLearner)...");
        // Logica fittizia per popolare `name_clusters`.
        self.name_clusters.insert("ClusterA".to_string(), NameCluster {
            cluster_id: 1,
            members: vec!["Mario".to_string(), "Mairo".to_string(), "Mara".to_string()],
            centroid: "Mario".to_string(),
            phonetic_signature: "MR".to_string(),
            confidence: 0.9,
        });
        self.name_clusters.insert("ClusterB".to_string(), NameCluster {
            cluster_id: 2,
            members: vec!["Giovanni".to_string(), "Gianni".to_string(), "Giova".to_string()],
            centroid: "Giovanni".to_string(),
            phonetic_signature: "JFN".to_string(),
            confidence: 0.85,
        });
        Ok(())
    }

    /// Mock: Identifica pattern nelle varianti basandosi sui dati osservati.
    /// Questo metodo non prende argomenti, ma opera sui dati già presenti nel learner
    /// (es. `pattern_frequency` o i cluster).
    /// (Dovrai implementare la logica reale qui per l'estrazione dei pattern).
    pub fn identify_variant_patterns(&mut self) -> Result<(), RitmoErr> {
        println!("    Identificazione dei pattern di varianti (logica fittizia in MLNameLearner)...");
        // Logica fittizia per popolare `learned_patterns`.
        // Questi pattern potrebbero derivare dall'analisi di `pattern_frequency` o `name_clusters`.
        self.learned_patterns.push(NameVariantPattern {
            base_form: "mario".to_string(),
            variant_form: "mairo".to_string(),
            pattern_type: VariantPatternType::Typo,
            confidence: 0.92,
            frequency: 5,
            phonetic_similarity: 0.8,
            edit_distance: 1,
        });
        self.learned_patterns.push(NameVariantPattern {
            base_form: "giovanni".to_string(),
            variant_form: "gianni".to_string(),
            pattern_type: VariantPatternType::Abbreviation,
            confidence: 0.98,
            frequency: 10,
            phonetic_similarity: 0.95,
            edit_distance: 2,
        });
        Ok(())
    }

    /// Mock: Trova una variante appresa tra due nomi dati.
    /// (Dovrai implementare la logica reale qui per cercare in `learned_patterns`).
    pub fn find_learned_variant(&self, input_name: &str, person_name: &str) -> Option<NameVariantPattern> {
        // Logica fittizia: simula la ricerca di un pattern specifico.
        // La tua implementazione reale dovrebbe iterare su `self.learned_patterns`
        // e confrontare `base_form` e `variant_form` con `input_name` e `person_name`.
        if (input_name == "mario" && person_name == "mairo") || (input_name == "mairo" && person_name == "mario") {
            Some(NameVariantPattern {
                base_form: "mario".to_string(),
                variant_form: "mairo".to_string(),
                pattern_type: VariantPatternType::Typo,
                confidence: 0.92,
                frequency: 5,
                phonetic_similarity: 0.8,
                edit_distance: 1,
            })
        } else if (input_name == "giovanni" && person_name == "gianni") || (input_name == "gianni" && person_name == "giovanni") {
            Some(NameVariantPattern {
                base_form: "giovanni".to_string(),
                variant_form: "gianni".to_string(),
                pattern_type: VariantPatternType::Abbreviation,
                confidence: 0.98,
                frequency: 10,
                phonetic_similarity: 0.95,
                edit_distance: 2,
            })
        }
        else {
            None
        }
    }

    /// Mock: Aggiunge una variante osservata, incrementando la frequenza di un pattern
    /// per l'apprendimento incrementale.
    /// (Dovrai implementare la logica reale qui per aggiornare `pattern_frequency`).
    pub fn add_observed_variant(&mut self, name1: &str, name2: &str, confidence: f64) -> Result<(), RitmoErr> {
        println!("    Aggiunta variante osservata: '{}' <-> '{}' (confidenza: {}) (logica fittizia in MLNameLearner)...", name1, name2, confidence);
        // Crea una chiave canonica per la coppia di nomi per tracciare la frequenza.
        let key = format!("{}-{}", name1.min(name2), name1.max(name2));
        *self.pattern_frequency.entry(key).or_default() += 1; // Incrementa il contatore.
        Ok(())
    }
}
