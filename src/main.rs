use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;

mod errors;
mod db;
mod tools;
mod import;

use tools::names_check::check_names;
use db::connection::establish_connection;
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
    
    /// Import data from a Calibre database
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
    },

    /// Shows couples of most resembling names, to check for typing errors or incomplete names
    Names {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
    },

    Check {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
    },

    Add {
        /// Database path
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
    },

    /// Add a new content entry
    AddContent {
        /// Database path
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
    },
}

#[tokio::main] 
async fn main() -> Result<(), RitmoErr> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path } => {
            let _conn = establish_connection(&path, true).await?;
        },
        Commands::Import { source, destination } => {
            let conn = establish_connection(&destination, true).await?;
            drop(conn);
            let conn = establish_connection(&source, false).await?;
            drop(conn);

            let _ = import_main::copy_data_from_calibre_db(&source, &destination).await?;

        },
        Commands::List { path } => {
            let db = establish_connection(&path, false).await?;
        }
        Commands::Names { path } => {
            let conn = establish_connection(&path, false).await?;
            let names = check_names(&conn, 0.96, 0.93).await?;
            for n in names {
                println!("{:?}", n);
            }            
        }
        Commands::Check { path: _ } => {
        }
        Commands::Add { path: _ } => {
        }
        Commands::AddContent { path: _ } => {
        }
    }
    Ok(())
}    
