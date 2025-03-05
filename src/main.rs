use crate::db::search::query_build::query_build;
use crate::db::adds::add_books::{add_book, BookData};
use crate::db::adds::add_contents::{add_content, ContentData};
use crate::db::do_filter::{get_book_ids_by_current_language, get_book_ids_by_person_name};
use crate::db::connection::create_pool;
use crate::errors::RitmoErr;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;

mod errors;
mod db;
mod tools;
mod import;

use tools::names_check::{check_names, compare_single_name};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about)]
#[command(name = "ritmo")]
#[command(version = "0.1.0")]
#[command(author = "Emanuele Ciarrocchi")]
#[command(about = "A CLI tool for database operations")]
#[command(long_about = "A comprehensive database management tool for organizing and manipulating books databases")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        #[arg(short, long, help = "Output path for the new database file", default_value = "../db001")]
        path: PathBuf,
    },
    Import {
        #[arg(short, long, help = "Path to the source database file", default_value = "../emalib_SSD/metadata.db")]
        source: PathBuf,
        #[arg(short, long, help = "Path to the destination database dir", default_value = "../db001")]
        destination: PathBuf,
    },
    List {
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
        #[arg(short, long, help = "Id number of book to read", default_value = "1")]
        id: i64,
    },
    Names {
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
    },
    Check {
        #[arg(short, long, help = "Path to the source database file", default_value = "../db001")]
        path: PathBuf,
        #[arg(short, long, help = "Name to compare", default_value = "Asimov Isaac")]
        name: String,
    },
    Search {
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
        #[arg(short, long, help = "Name to compare", default_value = "a")]
        name: String,
    },
    ContentAdd {
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
    },
    BookAdd {
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
    },
    Test {
    },
}

#[tokio::main]
async fn main() -> Result<(), RitmoErr> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path } => {
            create_pool(&path, true).await?;
        },
        Commands::Import { source, destination } => {
            let destination_pool = create_pool(&destination, true).await?;
            let source_pool = create_pool(&source, false).await?;
            import::copy_data_from_calibre_db(&source_pool, &destination_pool).await?;
        },
        Commands::List { path: _, id: _ } => {
            // Implementation for listing books can be added here
        },
        Commands::Names { path } => {
            let conn = create_pool(&path, false).await?;
            let names = check_names(&conn, 0.96, 0.93).await?;
            names.iter().for_each(|n| println!("{:?}", n));
        },
        Commands::Check { path, name } => {
            let conn = create_pool(&path, false).await?;
            let names = compare_single_name(&conn, name.clone(), 0.7, 0.7).await?;
            names.iter().for_each(|n| println!("{:?}", n));
        },
        Commands::Search { path, name } => {
            let pool = create_pool(&path, false).await?;
            println!("Searching database {:?} for {:?} books", path, name);
            let book_ids = get_book_ids_by_person_name(&pool, name).await?;
            println!("Found {:?} {} books", book_ids.len(), name);
            let english_books = get_book_ids_by_current_language(&pool, "eng").await?;
            println!("Found {:?} books in English", english_books.len());
        },
        Commands::ContentAdd { path } => {
            let pool = create_pool(&path, false).await?;
            let content_data = ContentData {
                name: "RACCONTO4".to_string(),
                original_title: Some("Original title 4".to_string()),
                publication_date: Some(1678886400),
                notes: Some("Additional notes".to_string()),
                type_id: Some("Novel".to_string()),
                curr_lang: vec!["Italian".to_string(), "Croatian".to_string()],
                orig_lang: vec!["English".to_string()],
                people: vec![
                    ("cino lino".to_string(), "Author".to_string()),
                    ("rino pino".to_string(), "Translator".to_string()),
                    ("mino nino".to_string(), "Cover designer".to_string()),
                    ("quell'altro".to_string(), "fancazzista".to_string()),
                ],
                tags: vec!["stronzata".to_string(), "altra stronzata".to_string()],
                ..Default::default()
            };
            match add_content(pool, &content_data).await {
                Ok(content_id) => println!("Content added with ID: {}", content_id),
                Err(e) => eprintln!("Error adding content: {}", e),
            }
        },
        Commands::BookAdd { path } => {
            let pool = create_pool(&path, false).await?;
            let book_data = BookData {
                name: "Libro".to_string(),
                format: Some("EPUB".to_string()),
                series: Some("Urania".to_string()),
                publisher: Some("Montatori".to_string()),
                ..Default::default()
            };
            match add_book(pool, &book_data).await {
                Ok(content_id) => println!("Book added with ID: {}", content_id),
                Err(e) => eprintln!("Error adding book: {}", e),
            }
        },
        Commands::Test {} => {
            println!("Test");
            let _ = query_build().await;
            println!("Done");
        },

    }
    Ok(())
}
