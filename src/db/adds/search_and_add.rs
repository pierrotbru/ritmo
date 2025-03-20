use sqlx::Transaction;
use sqlx::query_as;
use sqlx::query;
use crate::RitmoErr;
use sqlx::{Encode, Type, Sqlite, Database, encode::IsNull};
use sqlx::sqlite::SqliteArgumentValue;

pub enum IdAction {
    SearchId,
    AddId,
    RemoveId,
}

#[derive(sqlx::FromRow)]
struct GenericId {
    id: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct NewId {
    pub id: Option<i64>,
    pub added: bool,
}

impl<'q> Encode<'q, Sqlite> for NewId {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        match self.id {
            Some(id) => {
                args.push(SqliteArgumentValue::Int64(id));
                IsNull::No
            }
            None => IsNull::Yes,
        }
    }

    fn encode(self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        match self.id {
            Some(id) => {
                args.push(SqliteArgumentValue::Int64(id));
                IsNull::No
            }
            None => IsNull::Yes,
        }
    }

    fn size_hint(&self) -> usize {
        match self.id {
            Some(id) => <i64 as Encode<'q, Sqlite>>::size_hint(&id),
            None => 0,
        }
    }
}

impl Type<Sqlite> for NewId {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &<Sqlite as Database>::TypeInfo) -> bool {
        <i64 as Type<Sqlite>>::compatible(ty)
    }
}

pub async fn search_and_add(
    tx: &mut Transaction<'_, Sqlite>,
    table_name: &str,
    id_column: &str,
    name_column: &str,
    target: &str,
    add_it: IdAction,
) -> Result<NewId, RitmoErr> {
    // Validazione degli input (esempio base)
    if table_name.is_empty() || id_column.is_empty() || name_column.is_empty() || target.is_empty() {
        return Err(RitmoErr::InvalidInput("Input non validi".to_string()));
    }

    let select_query = format!("SELECT {} FROM {} WHERE {} = ?", id_column, table_name, name_column);

    let row: Option<GenericId> = query_as(&select_query)
        .bind(target)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nella query di selezione su {}: {}", table_name, e)))?;

    match add_it {
        IdAction::SearchId => {
            Ok(NewId { id: row.map(|r| r.id), added: false })
        }
        IdAction::AddId => {
            if let Some(existing_row) = row {
                Ok(NewId { id: Some(existing_row.id), added: false })
            } else {
                let insert_query = format!("INSERT INTO {} ({}) VALUES (?)", table_name, name_column);
                let result = query(&insert_query)
                    .bind(target)
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Errore nell'inserimento in {}: {}", table_name, e)))?;

                let inserted_id = result.last_insert_rowid();
                Ok(NewId { id: Some(inserted_id), added: true })
            }
        }
        IdAction::RemoveId => {
            if let Some(existing_row) = row {
                let delete_query = format!("DELETE FROM {} WHERE {} = ?", table_name, id_column);
                query(&delete_query)
                    .bind(existing_row.id)
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| RitmoErr::DatabaseDeleteFailed(format!("Errore nell'eliminazione da {}: {}", table_name, e)))?;
                Ok(NewId { id: Some(existing_row.id), added: true })
            } else {
                Err(RitmoErr::RecordNotFound)
            }
        }
    }
}
