use crate::db::adds::search_and_add::search_and_add;
use crate::db::adds::search_and_add::IdAction;
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
    .fetch_optional(&mut **tx) // Usa la transazione
    .await?;

    match result {
        Some(row) => Ok(row.iso_code),
        None => Err(RitmoErr::NoResultsError(format!(
            "Language '{}' not found, cazzo!",
            language_name
        ))),
    }
}


pub async fn add_languages(
    tx: &mut Transaction<'_, Sqlite>,
    new_content: &ContentData,
    _new_content_id: i64, // Usa i64 invece di u64
) -> Result<(), RitmoErr> {

    println!("new_content.curr_lang {:?}", &new_content.curr_lang);

    for curr in &new_content.curr_lang {
        let code = get_language_code_by_name(tx, curr).await?; // Aspetta il risultato e gestisci l'errore

        println!("{:?}",code );
        let curr_result = search_and_add(tx, "current_languages", "id", "iso_code", &code, IdAction::AddId,).await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add current language {}: {}", curr, e)))?;

        println!("{:?}",curr_result );
    }
    for curr in &new_content.src_lang {
        let code = get_language_code_by_name(tx, curr).await?; // Aspetta il risultato e gestisci l'errore

        println!("{:?}",code );
        let curr_result = search_and_add(tx, "source_languages", "id", "iso_code", &code, IdAction::AddId,).await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add current language {}: {}", curr, e)))?;

        println!("{:?}",curr_result );
    }
    for curr in &new_content.orig_lang {
        let code = get_language_code_by_name(tx, curr).await?; // Aspetta il risultato e gestisci l'errore

        println!("{:?}",code );
        let curr_result = search_and_add(tx, "original_languages", "id", "iso_code", &code, IdAction::AddId,).await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add current language {}: {}", curr, e)))?;

        println!("{:?}",curr_result );
    }

    Ok(())

}
