use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;

mod errors;
mod db;

use db::verify_path::verify_path;
use db::connection::establish_connection;

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
        source: PathBuf,
    },

    /// Shows couples of most resembling names, to check for typing errors or incomplete names
    Names {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        source: PathBuf,
    },

    Check {
        /// Source database path to import data from
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        source: PathBuf,
    },

    Add {
        /// Database path
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        source: PathBuf,
    },

}


#[tokio::main] 
async fn main() -> Result<(), RitmoErr> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { path } => {
            let db_path = verify_path(&path, true)?;
            let _connection = establish_connection(&db_path, true).await?;
        },
        Commands::Import { source: _, destination: _ } => {
        },
        Commands::List { source: _ } => {
        }
        Commands::Names { source: _ } => {
        }
        Commands::Check { source: _ } => {
        }
        Commands::Add { source: _ } => {
        }
    }
    Ok(())
}    
