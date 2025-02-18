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

pub async fn import_books_tags(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT book, tag FROM books_tags_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table tags: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::books_tags::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::books_tags::ActiveModel = crate::db::entity::books_tags::ActiveModel {
            book_id: Set(row.get(0)),
            tag_id: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = BooksTags::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import books_tags: {:?}", duration);
    Ok(())
}
