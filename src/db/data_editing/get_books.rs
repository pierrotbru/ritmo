use crate::db::entity::v_contents_details;
use sea_orm_migration::prelude::Query;
use crate::RitmoErr;
use sea_orm::{
    DatabaseConnection, 
    EntityTrait, 
    QueryFilter,
    ColumnTrait,
};
use sea_orm::sea_query::{Iden, SeaRc, ColumnRef, SimpleExpr};
use crate::db::entity::{
    v_books_details, 
    v_books_people_details, 
    v_books_with_contents,
};
use serde::{Deserialize, Serialize};

// Custom Iden implementation for table and column names
#[derive(Clone)]
enum BooksContents {
    Table,
    ContentId,
    BookId,
}

impl Iden for BooksContents {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            BooksContents::Table => "books_contents",
            BooksContents::ContentId => "content_id",
            BooksContents::BookId => "book_id",
        }).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookDetails {
    // Basic book details
    pub book: v_books_details::Model,
    
    // People involved with the book (authors, translators, etc.)
    pub people: Vec<v_books_people_details::Model>,
    
    // Contents in the book
    pub contents_ids: Vec<i64>,
    pub contents_titles: Vec<String>,
}

pub async fn get_book_details(
    db: &DatabaseConnection, 
    book_id: i64
) -> Result<Option<BookDetails>, RitmoErr> {
    // Fetch book details
    let book_details = v_books_details::Entity::find()
        .filter(v_books_details::Column::BookId.eq(book_id))
        .one(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;
    
    // If book doesn't exist, return None
    let book_details = match book_details {
        Some(details) => details,
        None => return Ok(None),
    };
    
    // Fetch people details for the book
    let people_details = v_books_people_details::Entity::find()
        .filter(v_books_people_details::Column::BookId.eq(book_id))
        .all(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;
    
    // Fetch contents for the book
    let books_with_contents = v_books_with_contents::Entity::find()
        .filter(v_books_with_contents::Column::BookId.eq(book_id))
        .one(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;
    
    // Extract contents information
    let (contents_ids, contents_titles) = books_with_contents
        .map(|bwc| (bwc.get_content_ids(), bwc.get_content_titles()))
        .unwrap_or_default();
    
    Ok(Some(BookDetails {
        book: book_details,
        people: people_details,
        contents_ids,
        contents_titles,
    }))
}

impl BookDetails {
    pub fn print_summary(&self) {
        println!("Book: {}", self.book.book_title);
        println!("Publisher: {}", self.book.publisher_name.as_deref().unwrap_or("Unknown"));
        println!("Series: {}", self.book.series_name.as_deref().unwrap_or("No Series"));
        println!("Publication Year: {}", self.book.publication_date.map(|y| y.to_string()).unwrap_or_else(|| "Unknown".to_string()));
        
        println!("\nPeople Involved:");
        for person in &self.people {
            println!("- {} ({})", person.person_name, person.role_name);
        }
        
        println!("\nContents:");
        for (id, title) in self.contents_ids.iter().zip(self.contents_titles.iter()) {
            println!("- {} (ID: {})", title, id);
        }
    }
}

pub async fn get_view_bwc(
    db: &DatabaseConnection, 
    book_id: i64
) -> Result<Option<v_books_with_contents::Model>, RitmoErr> {
    let book_detail = v_books_with_contents::Entity::find()
        .filter(v_books_with_contents::Column::BookId.eq(book_id))
        .one(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;

    Ok(book_detail)
}

pub async fn get_book_contents(
    db: &DatabaseConnection, 
    book_id: i64
) -> Result<Vec<v_contents_details::Model>, RitmoErr> {
    // First, check if the book exists
    let book = v_books_details::Entity::find()
        .filter(v_books_details::Column::BookId.eq(book_id))
        .one(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;

    // If book doesn't exist, return an empty vector
    if book.is_none() {
        return Ok(vec![]);
    }

    // Find contents for this book
    let contents = v_contents_details::Entity::find()
        .filter(
            v_contents_details::Column::ContentId.in_subquery(
                Query::select()
                    .column(ColumnRef::Column(SeaRc::new(BooksContents::ContentId)))
                    .from(SeaRc::new(BooksContents::Table))
                    .and_where(
                        SimpleExpr::Column(ColumnRef::Column(SeaRc::new(BooksContents::BookId)))
                        .eq(SimpleExpr::Value(book_id.into()))
                    )
                    .to_owned()
            )
        )
        .all(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;

    Ok(contents)
}
