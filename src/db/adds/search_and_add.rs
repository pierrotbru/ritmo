use sqlx::{Sqlite, Transaction, query, query_as};
use crate::RitmoErr;

#[derive(Debug)]
pub struct NewId {
    pub id: Option<i32>,
    pub added: bool,
}

pub enum IdAction {
    SearchId,
    AddId,
    RemoveId,
}

#[derive(sqlx::FromRow)]
struct GenericId {
    id: i32,
}

pub async fn search_and_add(
    tx: &mut Transaction<'_, Sqlite>,
    table_name: &str,
    id_column: &str,
    name_column: &str,
    target: &str,
    add_it: IdAction,
) -> Result<NewId, RitmoErr> {
    let query_str = format!("SELECT {} FROM {} WHERE {} = ?", id_column, table_name, name_column);
    let row: Option<GenericId> = query_as(&query_str)
        .bind(target)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    match add_it {
        IdAction::SearchId => {
            Ok(NewId { id: row.map(|r| r.id), added: false })
        }
        IdAction::AddId => {
            if row.is_some() {
                Ok(NewId { id: row.map(|r| r.id), added: false })
            } else {
                let insert_query_str = format!("INSERT INTO {} ({}) VALUES (?)", table_name, name_column);
                let result = query(&insert_query_str)
                    .bind(target)
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;

                let inserted_id = result.last_insert_rowid() as i32;
                Ok(NewId { id: Some(inserted_id), added: true })
            }
        }
        IdAction::RemoveId => {
            if let Some(row) = row {
                let delete_query_str = format!("DELETE FROM {} WHERE {} = ?", table_name, id_column);
                query(&delete_query_str)
                    .bind(row.id)
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| RitmoErr::DatabaseDeleteFailed(e.to_string()))?;
                Ok(NewId { id: Some(row.id), added: true })
            } else {
                Err(RitmoErr::RecordNotFound)
            }
        }
    }
}
