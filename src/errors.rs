#![allow(unused)]

use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum QueryBuilderError {
    #[error("Query building error: {0}")]
    InvalidParameter(&'static str),
    #[error("Query synthax error: {0}")]
    SyntaxError(String),
    #[error("No column selection in query")]
    NoSelectColumns,
    #[error("Query generic error: {0}")]
    GenericError(String),
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Query execution error: {0}")]
    GenericQueryError(String),
    #[error("Query building error: {0}")]
    QueryBuilderError(#[from] QueryBuilderError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
}

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
    #[error("Query building error: {0}")]
    QueryBuilderError(#[from] QueryBuilderError), 
    #[error("Invalid table name: {0}")]
    InvalidTableName(String),
    #[error("Invalid column name: {0}")]
    InvalidColumnName(String),
    #[error("Query execution error: {0}")]
    QueryError(#[from] QueryError),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Search and add operation failed: {0}")]
    SearchAndAddFailed(String),
    #[error("Transaction commit failed: {0}")]
    TransactionCommitFailed(String),
}

impl From<SqlxError> for RitmoErr {
    fn from(err: SqlxError) -> Self {
        RitmoErr::UnknownError(err.to_string())
    }
}
