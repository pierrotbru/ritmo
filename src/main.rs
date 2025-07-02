use crate::db::books::*;
use crate::db::contents::*;
use crate::db::connection::create_pool;
use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;

mod errors;
mod db;
mod import;
mod names;
mod publishers;
mod ml;


/// Gestore database per libreria digitale
/// 
/// Questo strumento ti permette di gestire database di libri digitali,
/// incluse operazioni di import, ricerca, e gestione contenuti.
#[derive(Parser)]
#[command(name = "ritmo")]
#[command(version = "0.1.0")]
#[command(author = "Emanuele Ciarrocchi")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crea un nuovo database vuoto
    /// 
    /// Inizializza una nuova struttura di database nella directory specificata.
    /// Se la directory non esiste, verrà creata automaticamente.
    New {
        /// Percorso di destinazione per il nuovo database
        /// 
        /// Specifica il percorso dove creare il nuovo database.
        /// Se la directory non esiste, verrà creata automaticamente.
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Forza la creazione sovrascrivendo file esistenti
        #[arg(long)]
        force: bool,
    },
    
    /// Importa dati da un database esistente
    /// 
    /// Copia e converte i dati da un database sorgente verso una nuova
    /// destinazione, mantenendo l'integrità dei dati originali.
    Import {
        /// Percorso del database sorgente da importare
        /// 
        /// Specifica il file database SQLite sorgente da cui importare i dati.
        /// Il file deve essere un database SQLite valido.
        #[arg(short, long, default_value = "../emalib_SSD/metadata.db")]
        source: PathBuf,
        
        /// Directory di destinazione per l'import
        /// 
        /// Directory dove salvare il database importato.
        /// Se non esiste, verrà creata automaticamente.
        #[arg(short, long, default_value = "../db001")]
        destination: PathBuf,
        
        /// Modalità verbosa per vedere i dettagli dell'import
        #[arg(long)]
        verbose: bool,
    },
    
    /// Elenca e visualizza informazioni sui libri
    /// 
    /// Mostra i dettagli di uno o più libri nel database,
    /// inclusi metadati e informazioni di archiviazione.
    List {
        /// Percorso del database da consultare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// ID specifico del libro da visualizzare
        /// 
        /// Specifica l'ID numerico del libro di cui visualizzare i dettagli.
        /// Usa 0 per elencare tutti i libri disponibili.
        #[arg(short, long, default_value_t = 1)]
        id: i64,
        
        /// Mostra informazioni dettagliate aggiuntive
        #[arg(long)]
        detailed: bool,
    },
    
    /// Elenca tutti gli autori presenti nel database
    /// 
    /// Visualizza un elenco completo di tutti gli autori catalogati,
    /// con statistiche opzionali sui loro libri.
    Names {
        /// Percorso del database da consultare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Mostra il conteggio dei libri per ogni autore
        #[arg(long)]
        count: bool,
        
        /// Ordina i risultati alfabeticamente
        #[arg(long)]
        sort: bool,
    },
    
    /// Verifica e confronta nomi di autori
    /// 
    /// Cerca corrispondenze e varianti di un nome autore nel database,
    /// utile per identificare duplicati o variazioni nella scrittura.
    Check {
        /// Percorso del database da consultare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Nome dell'autore da verificare
        /// 
        /// Nome completo dell'autore da cercare nel database.
        /// Il confronto è case-insensitive e supporta ricerche parziali.
        #[arg(short, long, default_value = "Asimov Isaac")]
        name: String,
        
        /// Ricerca fuzzy per trovare nomi simili
        #[arg(long)]
        fuzzy: bool,
    },
    
    /// Ricerca libri per titolo, autore o contenuto
    /// 
    /// Esegue ricerche full-text nel database per trovare libri
    /// che corrispondono ai criteri specificati.
    Search {
        /// Percorso del database da consultare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Termine di ricerca
        /// 
        /// Termine da cercare in titoli, autori e contenuti.
        /// Supporta ricerche parziali e operatori booleani.
        #[arg(short, long, default_value = "a")]
        query: String,
        
        /// Limita il numero di risultati mostrati
        #[arg(long, default_value_t = 10)]
        limit: usize,
        
        /// Cerca solo nei titoli
        #[arg(long)]
        title_only: bool,
        
        /// Cerca solo negli autori
        #[arg(long)]
        author_only: bool,
    },
    
    /// Aggiungi contenuto testuale a libri esistenti
    /// 
    /// Importa e associa contenuto testuale (OCR, estratti, etc.)
    /// a libri già presenti nel database.
    ContentAdd {
        /// Percorso del database da modificare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Percorso del file di contenuto da aggiungere
        #[arg(long)]
        content_file: Option<PathBuf>,
        
        /// ID del libro a cui associare il contenuto
        #[arg(long)]
        book_id: Option<i64>,
    },
    
    /// Aggiungi un nuovo libro al database
    /// 
    /// Registra un nuovo libro nel database con i suoi metadati,
    /// creando tutte le associazioni necessarie.
    BookAdd {
        /// Percorso del database da modificare
        #[arg(short, long, default_value = "../db001")]
        path: PathBuf,
        
        /// Titolo del libro
        #[arg(long)]
        title: Option<String>,
        
        /// Autore del libro
        #[arg(long)]
        author: Option<String>,
        
        /// Percorso del file del libro
        #[arg(long)]
        file_path: Option<PathBuf>,
        
        /// Modalità interattiva per inserire i dati
        #[arg(long)]
        interactive: bool,
    },
    
    /// Esegui test di integrità del database
    /// 
    /// Verifica la consistenza e l'integrità dei dati nel database,
    /// identificando eventuali problemi o corruzioni.
    Test {
        #[arg(short, long, default_value = "../emalib_SSD/metadata.db")]
        source: PathBuf,
        
        #[arg(short, long, default_value = "../db001")]
        destination: PathBuf,
    
    },
}

#[tokio::main]
async fn main() -> Result<(), RitmoErr> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path, force: _ } => {
            create_pool(&path, true).await?;
        },
        Commands::Import { source, destination, .. } => {
            let destination_pool = create_pool(&destination, true).await?;
            let source_pool = create_pool(&source, false).await?;
            import::copy_data_from_calibre_db(&source_pool, &destination_pool).await?;
        },
        Commands::List { path: _, id: _, .. } => {
        },
        Commands::Names {  .. } => {
        },
        Commands::Check {   .. } => {
        },
        Commands::Search { path: _, .. } => {
        },
        Commands::ContentAdd { path, .. } => {
            let pool = create_pool(&path, false).await?;
            let mut content = Content {
                data: ContentUserData {
                    name: "RACCONTO2".to_string(),
                    original_title: Some("Original title 2".to_string()),
                    publication_date: Some(1678886402),
                    notes: Some("Additional notes 2".to_string()),
                    type_id: Some("Novel".to_string()),
                    lang: vec![("Italian".to_string(),"current".to_string()), ("Russian".to_string(),"original".to_string())],
                    people: vec![
                        ("unknown".to_string(), "Author".to_string()),
                    ],
                    tags: vec!["boh".to_string()],
                    to_book: 1,
                    ..Default::default()
                }, 
                ..Default::default()
            };
            let _new_content_id = content.add_content(pool).await?;
        },
        Commands::BookAdd { path, .. } => {
            let pool = create_pool(&path, false).await?;


            let mut new_book = Book {
                data: BookUserData {
                    name: "Libro".to_string(),
                    format: Some("EPUB".to_string()),
                    series: Some("Urania".to_string()),
                    publisher: Some("Montatori".to_string()),
                    ..Default::default()
                }, 
                ..Default::default()
            };
            let content = Content {
                data: ContentUserData {
                    name: "RACCONTO4".to_string(),
                    original_title: Some("Original title 4".to_string()),
                    publication_date: Some(1678886400),
                    notes: Some("Additional notes".to_string()),
                    type_id: Some("Novel".to_string()),
                    lang: vec![("Italian".to_string(),"current".to_string()), ("Croatian".to_string(),"original".to_string())],
                    people: vec![
                        ("cino lino".to_string(), "Author".to_string()),
                        ("rino pino".to_string(), "Translator".to_string()),
                        ("mino nino".to_string(), "Cover designer".to_string()),
                        ("quell'altro".to_string(), "fancazzista".to_string()),
                    ],
                    tags: vec!["stronzata".to_string(), "altra stronzata".to_string()],
                    ..Default::default()
                }, 
                ..Default::default()
            };
            new_book.data.contents.push(content);
            
            let content = Content {
                data: ContentUserData {
                    name: "RACCONTO3".to_string(),
                    original_title: Some("Original title 3".to_string()),
                    publication_date: Some(1678886401),
                    notes: Some("Additional notes 3".to_string()),
                    type_id: Some("Novel".to_string()),
                    lang: vec![("Italian".to_string(),"current".to_string()), ("Swedish".to_string(),"original".to_string())],
                    people: vec![
                        ("rino gino".to_string(), "Author".to_string()),
                        ("quell'altro".to_string(), "fancazzista".to_string()),
                    ],
                    tags: vec!["doppia stronzata".to_string(), "altra stronzata".to_string()],
                    ..Default::default()
                }, 
                ..Default::default()
            };
            new_book.data.contents.push(content);

            let _new_book_id = new_book.add_book(pool).await?;
        },
        Commands::Test { source, destination, .. } => {
            dbg!(source);
            dbg!(destination);
            let destination_pool = create_pool(&destination, true).await?;
            let source_pool = create_pool(&source, false).await?;
        },

    }
    Ok(())
}

