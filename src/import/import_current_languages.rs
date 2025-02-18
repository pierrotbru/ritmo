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

pub async fn import_current_languages(src: &SqlitePool, dst: &SqlitePool) -> Result<(), RitmoErr> {

    let ritmo_conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(dst.clone());

    let start = Instant::now();

    let calibre_rows = sqlx::query("SELECT id, lang_code FROM languages")
        .fetch_all(src)
        .await
        .map_err(|e| RitmoErr::ImportError(
            format!("Failed to fetch rows for table languages: {}", e)
        ))?;

    let mut table : Vec<crate::db::entity::current_languages::ActiveModel> = Vec::new();

    for row in calibre_rows {
        let single: crate::db::entity::current_languages::ActiveModel = crate::db::entity::current_languages::ActiveModel {
            id: Set(row.get(0)),
            lang_code: Set(row.get(1)),
            ..Default::default()
        };
        table.push(single);
    }

    let _ = CurrentLanguages::insert_many(table).exec(&ritmo_conn).await?;

    let duration = start.elapsed();
    println!("SeaORM import current_languages: {:?}", duration);
    Ok(())
}
