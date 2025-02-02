use thiserror::Error;
use sqlx::{Error as SqlxError};

#[derive(Error, Debug)]
pub enum QueryBuilderError {
    #[error("Errore durante la costruzione della query: {0}")]
    InvalidParameter(&'static str),
    #[error("Errore di sintassi della query: {0}")]
    SyntaxError(String),
    #[error("Nessuna colonna selezionata nella query")]
    NoSelectColumns,
    #[error("Errore generico nella query: {0}")]
    GenericError(String),
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Errore durante l'esecuzione della query: {0}")]
    GenericQueryError(String),
    #[error("Errore durante la costruzione della query: {0}")]
    QueryBuilderError(#[from] QueryBuilderError),
    #[error("Errore del database: {0}")]
    DatabaseError(#[from] SqlxError),
    // ... altri errori specifici
}

#[derive(Error, Debug)]
pub enum RitmoErr {
    #[error("Errore di accesso al file: {0}")]
    FileAccessError(#[from] std::io::Error),
    #[error("Nessun risultato trovato: {0}")]
    NoResultsError(String),
    #[error("Errore di integrità del database: {0}")]
    DataIntegrityError(String),
    #[error("Errore di importazione: {0}")]
    ImportError(String),
    #[error("Errore di esportazione: {0}")]
    ExportError(String),
    #[error("Errore sconosciuto: {0}")]
    UnknownError(String),
    #[error("Errore di percorso: {0}")]
    PathError(String),
    #[error("Errore di creazione database: {0}")]
    DatabaseCreationFailed(String),
    #[error("Altro errore: {0}")]
    OtherError(String),
    #[error("Errore durante la costruzione della query: {0}")]
    QueryBuilderError(#[from] QueryBuilderError), 
    #[error("Nome della tabella non valido: {0}")]
    InvalidTableName(String),
    #[error("Nome della colonna non valido: {0}")]
    InvalidColumnName(String),
    #[error("Errore durante l'esecuzione della query: {0}")]
    QueryError(#[from] QueryError),
}

impl From<SqlxError> for RitmoErr {
    fn from(err: SqlxError) -> Self {
        RitmoErr::UnknownError(err.to_string())
    }
}

