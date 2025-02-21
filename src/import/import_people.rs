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

pub async fn import_people(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, name FROM authors")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table people: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::people::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::people::ActiveModel = crate::db::entity::people::ActiveModel {
            name: Set(row.get(1)),
            id: Set(row.get(0)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = People::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import people: {:?}", duration);
    Ok(())
}
