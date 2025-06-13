//#![allow(unused)]
//
//use thiserror::Error;
//use sqlx::Error as SqlxError; // Alias per chiarezza
//
//// ==============================================================================
//// 1. Errori Specifici della Costruzione Query
////    Contengono i dettagli sul perché la costruzione della query è fallita.
////    Possono includere informazioni sul file/riga se utile per la debug.
//// ==============================================================================
//#[derive(Error, Debug)]
//pub enum QueryBuilderError {
//    #[error("Parametro non valido: '{param}' (File: {file}, Linea: {line})")]
//    InvalidParameter {
//        param: &'static str,
//        file: &'static str,
//        line: u32,
//    },
//    #[error("Errore di sintassi query: {syntax_error} (File: {file}, Linea: {line})")]
//    SyntaxError {
//        syntax_error: String,
//        file: &'static str,
//        line: u32,
//    },
//    #[error("Nessuna colonna selezionata nella query (File: {file}, Linea: {line})")]
//    NoSelectColumns {
//        file: &'static str,
//        line: u32,
//    },
//    #[error("Errore generico di costruzione query: {message} (File: {file}, Linea: {line})")]
//    GenericError {
//        message: String,
//        file: &'static str,
//        line: u32,
//    },
//    #[error("Nome tabella non valido: '{0}'")]
//    InvalidTableName(String),
//    #[error("Nome colonna non valido: '{0}'")]
//    InvalidColumnName(String),
//}
//
//// Funzione helper per creare QueryBuilderError con contesto file/linea
//#[macro_export]
//macro_rules! qb_err_invalid_param {
//    ($param:expr) => {
//        $crate::QueryBuilderError::InvalidParameter {
//            param: $param,
//            file: file!(),
//            line: line!(),
//        }
//    };
//}
//#[macro_export]
//macro_rules! qb_err_syntax {
//    ($msg:expr) => {
//        $crate::QueryBuilderError::SyntaxError {
//            syntax_error: $msg.to_string(),
//            file: file!(),
//            line: line!(),
//        }
//    };
//}
//// Potresti creare macro simili per altri errori se vuoi sempre file/linea
//
//// ==============================================================================
//// 2. Errori di Query (Esecuzione)
////    Wrapper per gli errori di costruzione query e gli errori diretti di sqlx.
////    Questo diventa il punto di convergenza per tutti gli errori legati alle query.
//// ==============================================================================
//#[derive(Error, Debug)]
//pub enum QueryError {
//    #[error("Errore di costruzione query: {0}")]
//    Build(#[from] QueryBuilderError), // QueryBuilderError viene convertito qui
//    #[error("Errore del database SQLx: {0}")]
//    Database(#[from] SqlxError), // SqlxError viene convertito qui
//    #[error("Errore generico di esecuzione query: {0}")]
//    Generic(String),
//}
//
//// ==============================================================================
//// 3. RitmoErr: Gli Errori dell'Applicazione a Livello Superiore
////    Questo è il tipo di errore principale che le tue funzioni di alto livello
////    restituiranno. Contiene gli errori più specifici attraverso la composizione.
//// ==============================================================================
//#[derive(Error, Debug)]
//pub enum RitmoErr {
//    // --- Errori legati al Database ---
//    #[error("Connessione al database fallita: {0}")]
//    DatabaseConnectionFailed(String), // Più specifico per la connessione iniziale
//    #[error("Migrazione del database fallita: {0}")]
//    DatabaseMigrationFailed(String), // Più specifico per le migrazioni
//    #[error("Creazione database fallita: {0}")]
//    DatabaseCreationFailed(String),
//    #[error("Transazione database fallita: {0}")]
//    DatabaseTransactionFailed(String), // Include commit, rollback, etc.
//    #[error("Errore di integrità dei dati nel database: {0}")]
//    DataIntegrityError(String),
//    #[error("Record non trovato nel database.")]
//    RecordNotFound,
//
//    // --- Errori di Query (Build & Execute) ---
//    // Tutto ciò che riguarda la costruzione o l'esecuzione delle query passa per qui.
//    #[error("Errore di query: {0}")]
//    Query(#[from] QueryError),
//
//    // --- Errori di I/O e File System ---
//    #[error("Errore di accesso al file/I/O: {0}")]
//    FileAccessError(#[from] std::io::Error), // Copre tutti gli errori IoError(String)
//    #[error("Percorso non valido: {0}")]
//    PathError(String),
//
//    // --- Errori di Logica di Business/Applicazione ---
//    #[error("Nessun risultato trovato per la ricerca: {0}")]
//    NoResultsFound(String), // Rinominato da NoResultsError
//    #[error("Errore durante l'importazione dati: {0}")]
//    ImportError(String),
//    #[error("Errore durante l'esportazione dati: {0}")]
//    ExportError(String),
//    #[error("Operazione 'Search and Add' fallita: {0}")]
//    SearchAndAddFailed(String),
//    #[error("Input non valido per l'operazione 'Search and Add': {0}")]
//    InvalidInput(String),
//    #[error("Errore durante il parsing del nome: {0}")]
//    NameParsingError(String),
//    #[error("Errore durante l'unione/merging dei nomi: {0}")]
//    MergeError(String),
//    #[error("Errore di Machine Learning: {0}")]
//    MLError(String),
//
//    // --- Errori Generici o Di Contingenza ---
//    // Usa `anyhow::Error` per errori "catch-all" che non rientrano nelle altre categorie
//    // e per mantenere la catena di cause di errori esterni non specifici.
//    #[error("Errore generico inatteso: {0}")]
//    Other(#[from] anyhow::Error), // Cattura un'ampia varietà di errori
//}
//

#![allow(unused)]

use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum RitmoErr {
    #[error("Migration failed: {0}")]
    DatabaseMigrationFailed(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("sqlx error: {0}")]
    SqlxError(sqlx::Error),
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(String),
    #[error("Database query failed: {0}")]
    DatabaseQueryFailed(String),
    #[error("Database insert failed: {0}")]
    DatabaseInsertFailed(String),
    #[error("Database delete failed: {0}")]
    DatabaseDeleteFailed(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("DB Transaction failed: {0}")]
    DatabaseTransactionError(String),
    #[error("File access failed: {0}")]
    FileAccessError(#[from] std::io::Error),
    #[error("No result found: {0}")]
    NoResultsError(String),
    #[error("Database integrity fail : {0}")]
    DataIntegrityError(String),
    #[error("Import error: {0}")]
    ImportError(String),
    #[error("Export error: {0}")]
    ExportError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Path error: {0}")]
    PathError(String),
    #[error("Database creation fail: {0}")]
    DatabaseCreationFailed(String),
    #[error("Other error: {0}")]
    OtherError(String),
//    #[error("Query building error: {0}")]
//    QueryBuilderError(#[from] QueryBuilderError), 
    #[error("Invalid table name: {0}")]
    InvalidTableName(String),
    #[error("Invalid column name: {0}")]
    InvalidColumnName(String),
//    #[error("Query execution error: {0}")]
//    QueryError(#[from] QueryError),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Search and add operation failed: {0}")]
    SearchAndAddFailed(String),
    #[error("Search and add invalid input: {0}")]
    InvalidInput(String),
    #[error("Transaction commit failed: {0}")]
    TransactionCommitFailed(String),
    #[error("Name parsing error: {0}")]
    NameParsingError(String),
    #[error("Name merging error: {0}")]
    MergeError(String),
    #[error("Errore di Machine Learning: {0}")]
    MLError(String),

}

// Implementazione per SqlxError
impl From<sqlx::Error> for RitmoErr {
    fn from(err: sqlx::Error) -> Self {
        // Ora puoi mappare SqlxError a DatabaseError
        RitmoErr::DatabaseError(format!("Database operation failed: {}", err))
        // Volendo, potresti anche fare:
        // RitmoErr::DatabaseError(err) // Se vuoi mantenere l'oggetto SqlxError
        // In tal caso, la variante sarebbe DatabaseError(sqlx::Error)
    }
}

// Implementazione per serde_json::Error
impl From<serde_json::Error> for RitmoErr {
    fn from(err: serde_json::Error) -> Self {
        RitmoErr::MLError(format!("Serialization/Deserialization error: {}", err))
        // Oppure RitmoErr::UnknownError(...) o un nuovo RitmoErr::SerializationError(...)
    }
}