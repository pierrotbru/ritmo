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

async fn insert_language(
    tx: &mut Transaction<'_, Sqlite>,
    table: &str,
    lang: &str,
    new_content_id: i64,
) -> Result<(), RitmoErr> {
    let code = get_language_code_by_name(tx, lang).await?;
    let code_id = search_and_add(tx, table, "id", "iso_code", &code, IdAction::AddId)
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add {}: {}", lang, e)))?;

    let query_str = format!("INSERT INTO contents_{} (content_id, lang_id) VALUES (?, ?)", table);

    let query = sqlx::query(&query_str).bind(new_content_id).bind(code_id.id);

    println!("{:?}, {:?}, {:?}",query_str, new_content_id, code );

    query.execute(&mut **tx)
        .await
        .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert into {}: {}", table, e)))?;

    Ok(())
}

pub async fn add_languages(
    tx: &mut Transaction<'_, Sqlite>,
    new_content: &ContentData,
    new_content_id: i64,
) -> Result<(), RitmoErr> {
    for lang in &new_content.curr_lang {
        insert_language(tx, "current_languages", lang, new_content_id).await?;
    }
    for lang in &new_content.src_lang {
        insert_language(tx, "source_languages", lang, new_content_id).await?;
    }
    for lang in &new_content.orig_lang {
        insert_language(tx, "original_languages", lang, new_content_id).await?;
    }

    Ok(())
}