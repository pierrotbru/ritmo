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


pub async fn sync_publishers(calibre_conn: &SqlitePool, my_conn: &SqlitePool) -> Result<(), RitmoErr> {

    let start = Instant::now();
    let publisher_links = sqlx::query("SELECT book, publisher FROM books_publishers_link")
        .fetch_all(calibre_conn)
        .await
        .map_err(|e| RitmoErr::ImportError(format!("Failed to fetch publisher links from Calibre: {}", e)))?;

    let duration = start.elapsed();
    println!("fetch editors: {:?}", duration);

    let start = Instant::now();
    let mut tx = my_conn.begin().await?;

    // Raggruppa gli aggiornamenti per batch (es. 1000 alla volta)
    for chunk in publisher_links.chunks(1000) {
        let mut book_updates = Vec::new();

        for link in chunk {
            let book_id: i64 = link.try_get("book")?;
            let publisher_id: i64 = link.try_get("publisher")?;
            book_updates.push((book_id, publisher_id));
        }

        // Esegui l'aggiornamento batch
        for (book_id, publisher_id) in book_updates {
            let _ = sqlx::query("UPDATE Books SET publisher_id = ? WHERE id = ?")
                .bind(publisher_id)
                .bind(book_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;
    let duration = start.elapsed();
    println!("update books: {:?}", duration);

    Ok(())
}

pub async fn import_publishers(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM publishers")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table publishers: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::publishers::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::publishers::ActiveModel = crate::db::entity::publishers::ActiveModel {
            id: Set(row.get(0)),
            name: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Publishers::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import publishers: {:?}", duration);
    Ok(())
}

