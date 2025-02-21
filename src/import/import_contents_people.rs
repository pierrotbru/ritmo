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

pub async fn import_contents_people(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT book, author FROM books_authors_link")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table tags: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::contents_people_roles::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::contents_people_roles::ActiveModel = crate::db::entity::contents_people_roles::ActiveModel {
            content_id: Set(row.get(0)),
            person_id: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = ContentsPeopleRoles::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import contents_people: {:?}", duration);
    Ok(())
}
