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

pub async fn import_tags(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM tags")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table tags: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::tags::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::tags::ActiveModel = crate::db::entity::tags::ActiveModel {
            id: Set(row.get(0)),
            tag_name: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = Tags::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import tags: {:?}", duration);
    Ok(())
}
