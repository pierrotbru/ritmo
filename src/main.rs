use crate::db::do_filter::get_book_ids_by_current_language;
use crate::db::do_filter::get_book_ids_by_person_name;
use crate::db::connection::create_pool;
use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;
//use tracing_subscriber;
//use tracing;

mod errors;
mod db;
mod tools;
mod import;

use tools::names_check::{check_names, compare_single_name};
use crate::import::import_main;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about)]
#[command(name = "ritmo")]
#[command(version = "0.1.0")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(about = "A CLI tool for database operations")]
#[command(long_about = "A comprehensive database management tool for organizing and manipulating books databases")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Create a new database
    New {
        /// Path where the new database will be created
        #[arg(short, long, help = "Output path for the new database file", default_value = "../db001")]
        path: PathBuf,
    },
    
    /// Creates a new database, then import data from a Calibre database
    Import {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../emalib_SSD/metadata.db")]
        source: PathBuf,
        
        /// Destination database path to import data to
        #[arg(short, long, help = "Path to the destination database dir", default_value = "../db001")]
        destination: PathBuf,
    },

    /// Read the list of books from a database
    List {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,

        /// number of book to list
        #[arg(short, long, help = "Id number of book to read", default_value = "1")]
        id: i64,
    },

    /// Shows couples of most resembling names, to check for typing errors or incomplete names
    Names {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
    },
    /// Check the names compare function, with a single name
    Check {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,

        #[arg(short, long, help = "Name to compare", default_value = "Asimov Isaac")]
        name: String,
    },
    /// Search the books for an author
    Search {
        /// Database path
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,

        #[arg(short, long, help = "Name to compare", default_value = "a")]
        name: String,
    },
    /// Add one book to the database
    Add {
        /// Database path
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
    }
}

#[tokio::main] 
async fn main() -> Result<(), RitmoErr> {

//    tracing_subscriber::fmt()
//        .with_max_level(tracing::Level::DEBUG)
//        .with_target(false)
//        .with_thread_ids(true)
//        .with_thread_names(true)
//        .with_file(true)
//        .with_line_number(true)
//        .init();
//    tracing_subscriber::fmt()
//        .with_max_level(tracing::Level::DEBUG)
//        .with_target(false)
//        .with_env_filter("sqlx=debug")
//        .init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path } => {
            let _conn = create_pool(&path, true).await?;
        },
      Commands::Import { source, destination } => {
            let destination = create_pool(&destination, true).await?;
            let source = create_pool(&source, false).await?;

            let _ = import_main::copy_data_from_calibre_db(&source, &destination).await?;

        },
        Commands::List { path: _ , id: _ } => {
        }
        Commands::Names { path } => {
            let conn = create_pool(&path, false).await?;
            let names = check_names(&conn, 0.96, 0.93).await?;
            for n in names {
                println!("{:?}", n);
            }            
        }
        Commands::Check { path, name} => {
            let conn = create_pool(&path, false).await?;
            let names = compare_single_name(&conn, (&name).to_string(), 0.7, 0.7).await?;
            for n in names {
                println!("{:?}", n);
            }            
        }
        Commands::Search { path, name } => {
            let pool = create_pool(&path, false).await?;

            println!("Searching database {:?} for {:?} books", path, name );

            let book_ids = get_book_ids_by_person_name(&pool, name).await?;
            println!("Found {:?} {} books", book_ids.len(), name);

            let book_ids = get_book_ids_by_current_language(&pool, "eng").await?;
            println!("Found {:?} books in english", book_ids.len());
        }
        Commands::Add {path} => {
            let _pool = create_pool(&path, false).await?;
        }
    }
    Ok(())
}    
