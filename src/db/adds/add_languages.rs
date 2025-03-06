use sqlx::query;
use crate::db::adds::search_and_add::{search_and_add, IdAction};
use crate::ContentData;
use crate::RitmoErr;
use sqlx::{query_as, Sqlite, Transaction};

pub async fn get_language_code_by_name(
    tx: &mut Transaction<'_, Sqlite>,
    language_name: &str,
) -> Result<String, RitmoErr> {
    #[derive(sqlx::FromRow)]
    struct IsoCode {
        iso_code: String,
    }

    let result = query_as!(
        IsoCode,
        "SELECT iso_code FROM languages_names WHERE TRIM(name) = TRIM(?)",
        language_name
    )
    .fetch_optional(&mut **tx) // Use the transaction directly
    .await?;

    result.map_or_else(
        || Err(RitmoErr::NoResultsError(format!("Language '{}' not found.", language_name))),
        |row| Ok(row.iso_code),
    )
}

pub async fn add_languages(
    tx: &mut Transaction<'_, Sqlite>,
    lang: Vec<(String, i32)>,
    new_content_id: i64,
) -> Result<(), RitmoErr> {

    for (iso_code, id_role) in lang {

        let mut code = String::new();
        if iso_code.len() != 3 {
            code = get_language_code_by_name(tx, &iso_code).await?;
            println!("ahia");
        }
        else {
            code = iso_code;
        }
        let code_id = search_and_add(tx, "running_languages", "id", "iso_code", &code, IdAction::AddId)
            .await
            .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add {}: {}", code, e)))?;
        let query = query!("UPDATE running_languages SET role = ? WHERE id = ?", id_role, code_id.id);
        query.execute(&mut **tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to update running_languages: {}", e)))?;

        let query = query!("INSERT INTO contents_languages (contents_id, languages_id) VALUES (?, ?)",new_content_id, code_id.id);
        query.execute(&mut **tx)
            .await
            .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert into running_languages: {}", e)))?;
    }

    Ok(())
}