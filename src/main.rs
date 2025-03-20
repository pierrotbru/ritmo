use crate::db::books::*;
use crate::db::contents::*;
use crate::db::search::query_build::BookSearchCriteria;
use crate::db::search::query_build::search_books;
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
        #[arg(short, long, help = "Path to the database file", default_value = "../db001")]
        path: PathBuf,
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
        Commands::Search { path: _, name: _ } => {
        },
        Commands::ContentAdd { path } => {
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
        Commands::BookAdd { path } => {
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

            let new_book_id = new_book.add_book(pool).await?;
        },
        Commands::Test { path } => {
            let pool = create_pool(&path, false).await?;
            let criteria = BookSearchCriteria {
                person_name_content: Some("Isaac Asimov".to_string()),
                ..Default::default()
            };
            let n = search_books(&pool, &criteria).await?;
            println!("found {:?} books", n);
        },

    }
    Ok(())
}
