use crate::db::adds::search_and_add::search_and_add;
use sqlx::query;
use crate::db::adds::search_and_add::IdAction;
use crate::ContentData;
use crate::RitmoErr;
use sqlx::{query_as, Sqlite, Transaction};

pub async fn get_language_code_by_name<'a>(
    tx: &'a mut Transaction<'a, Sqlite>,
    language_name: &'a str,
) -> Result<String, RitmoErr> {
    let language_name_pattern = format!("%{}%", language_name);

    #[derive(sqlx::FromRow)]
    struct IsoCode {
        iso_code: String,
    }

    let result = query_as!(
        IsoCode,
        "SELECT iso_code FROM languages_names WHERE name LIKE ?",
        language_name_pattern
    )
    .fetch_optional(&mut **tx) // Usa la transazione
    .await?;

    match result {
        Some(row) => Ok(row.iso_code),
        None => Err(RitmoErr::NoResultsError(format!(
            "Language '{}' not found",
            language_name
        ))),
    }
}


//pub async fn add_languages_join(
//    tx: &mut Transaction<'_, Sqlite>,
//    new_content: &ContentData,
//    new_content_id: i64, // Usa i64 invece di u64
//) -> Result<(), RitmoErr> {
//    for curr in &new_content.curr_lang {
//        let code = get_language_code_by_name(tx, curr).await?; // Aspetta il risultato e gestisci l'errore
//        let curr_result = //search_and_add(
//            &mut tx,
//            "Current_languages",
//            "id",
//            "language_id",
//            //code,
//            IdAction::AddId,
//        )
//        .await
//        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add current language {}: {}", curr, e)))?;
//
//        query!(
//            "INSERT INTO contents_current_languages (content_id, curr_lang_id) VALUES (?, ?)",
//            new_content_id,
//            curr_result
//        )
//        .execute(&mut **tx)
//        .await
//        .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
//    }
//    Ok(())
//
//}
