use sea_orm::{
    Set,
    EntityTrait,
    SqlxSqliteConnector
};
use sqlx::{
    SqlitePool, 
    Row
};
use crate::RitmoErr;
use std::time::Instant;
use crate::db::entity::prelude::*;

pub async fn import_books(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, title FROM books")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table books: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::books::ActiveModel> = Vec::new();
    for row in &calibre_rows {
        let single: crate::db::entity::books::ActiveModel = crate::db::entity::books::ActiveModel {
            id: Set(row.get(0)),
            title: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Books::insert_many(table).exec(&ritmo_conn).await?;

    let mut table : Vec<crate::db::entity::contents::ActiveModel> = Vec::new();
    for row in &calibre_rows {
        let single: crate::db::entity::contents::ActiveModel = crate::db::entity::contents::ActiveModel {
            id: Set(row.get(0)),
            title: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Contents::insert_many(table).exec(&ritmo_conn).await?;

    let mut table : Vec<crate::db::entity::books_contents::ActiveModel> = Vec::new();
    for row in calibre_rows {
        let single: crate::db::entity::books_contents::ActiveModel = crate::db::entity::books_contents::ActiveModel {
            book_id: Set(row.get(0)),
            content_id: Set(row.get(0)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = BooksContents::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import books: {:?}", duration);
    Ok(())
}

