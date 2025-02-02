use crate::filters::constants::constants::*;
use crate::db::query_conditions::{QueryCondition, LikeType};
use crate::db::query_builder::*;
use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;
use crate::db::connection::connect::connect_to_db;
use crate::db::{db_fill, db_schema};
use crate::tools::names_check;
use crate::import::import_main;

mod filters;
mod errors;
mod db;
mod import;
mod tools;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about)]
#[command(name = "ritmo")]
#[command(version = "0.1.0")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(about = "A CLI tool for database operations")]
#[command(long_about = "A comprehensive database management tool for organizing and manipulating book databases")]
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
            let conn = connect_to_db(&path, true).await?;
            let _ = db_schema::create_db_schema(&conn).await?;
            let _ = db_fill::fill_db(&conn).await?;
            println!("database {:?} created", path);
        },
        Commands::Import { source, destination } => {
            let s_conn = connect_to_db(&source, false).await?;
            let d_conn = connect_to_db(&destination, true).await?;
            let _ = db_schema::create_db_schema(&d_conn).await?;
            let _ = db_fill::fill_db(&d_conn).await?;
            println!("database {:?} created", destination);
            let _ = import_main::copy_data_from_calibre_db(&s_conn, &d_conn).await?;
        },
        Commands::List { source } => {
            let _conn = connect_to_db(&source, false).await?;
        }

        Commands::Names { source } => {
            let conn = connect_to_db(&source, false).await?;
            let names = names_check::check_names(&conn, 0.96, 0.93).await?;
            for n in names {
                println!("{:?}", n);
            }            

            let names = names_check::check_publ(&conn, 0.9, 0.86).await?;
            for n in names {
                println!("{:?}", n);
            }            
        }
        Commands::Check { source: _ } => {
            
            let _query = QueryBuilder::new(TABLE_BOOKS)
                .select_columns(&[COLUMN_BOOKS_ID])
                .add_condition(QueryCondition::Like("author".to_string(), "Tolkien".to_string(), LikeType::Contains));
        }
        Commands::Add { source: _ } => {
            
            let _query = QueryBuilder::new("Books")
                .select_columns(&["title", "author"])
                .add_condition(QueryCondition::Like("author".to_string(), "Tolkien".to_string(), LikeType::Contains));
        }
    }
    Ok(())
}    



