use crate::db::adds::add_languages::add_languages;
use crate::db::adds::search_and_add::*;
use crate::RitmoErr;
use sqlx::{query, SqlitePool};

#[derive(Debug, Default)]
pub struct ContentData {
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub notes: Option<String>,
    pub type_id: Option<String>,
    pub lang: Vec<(String,i32)>,
    pub people: Vec<(String, String)>,
    pub tags: Vec<String>,
}

#[derive(Debug, Default)]
pub struct NewContent {
    pub id: i32,
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub notes: Option<String>,
    pub type_id: Option<i32>,
    pub pre_accepted: i32,
}

#[derive(sqlx::FromRow)]
struct IdResult {
    id: i32,
}

pub async fn add_content(pool: SqlitePool, content: &ContentData) -> Result<i32, RitmoErr> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))?;

    let mut new_content = NewContent {
        name: content.name.clone(),
        original_title: content.original_title.clone(),
        publication_date: content.publication_date.clone(),
        notes: content.notes.clone(),
        ..Default::default()
    };

    if let Some(type_id) = &content.type_id {
        let type_result = search_and_add(
            &mut tx,
            "types",
            "id",
            "name",
            type_id,
            IdAction::AddId,
        )
        .await
        .map_err(|e| {
            RitmoErr::SearchAndAddFailed(format!("Failed to search and add type_id {}: {}", type_id, e))
        })?;
        new_content.type_id = type_result.id;
    }

    let result = query!(
        "INSERT INTO contents (name, original_title, publication_date, notes, type_id) VALUES (?, ?, ?, ?, ?)",
        new_content.name,
        new_content.original_title,
        new_content.publication_date,
        new_content.notes,
        new_content.type_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert content: {}", e)))?;

    let new_content_id = result.last_insert_rowid();

    println!("add_languages");
    let _ = add_languages(&mut tx, content.lang.clone(), new_content_id).await?;

    for tag in &content.tags {
        let tag_result = search_and_add(
            &mut tx,
            "tags",
            "id",
            "name",
            tag,
            IdAction::AddId,
        )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add tag {}: {}", tag, e)))?;

        let tag_id = tag_result.id.ok_or(RitmoErr::DatabaseInsertFailed("Tag not found or added".to_string()))?;

        query!(
            "INSERT INTO contents_tags (content_id, tag_id) VALUES (?, ?)",
            new_content_id,
            tag_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| RitmoErr::DatabaseInsertFailed(format!("Failed to insert content tags: {}", e)))?;
    }

    for (person_name, role_name) in &content.people {
        let person_result = search_and_add(
            &mut tx,
            "people",
            "id",
            "name",
            person_name,
            IdAction::AddId,
        )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add person {}: {}", person_name, e)))?;

        let person_id = person_result.id.ok_or(RitmoErr::DatabaseInsertFailed(format!(
            "Person {} not found or added",
            person_name
        )))?;

        let role_result = search_and_add(
            &mut tx,
            "roles",
            "id",
            "name",
            role_name,
            IdAction::AddId,
        )
        .await
        .map_err(|e| RitmoErr::SearchAndAddFailed(format!("Failed to search and add role {}: {}", role_name, e)))?;

        let role_id = role_result.id.ok_or(RitmoErr::DatabaseInsertFailed(format!(
            "Role {} not found or added",
            role_name
        )))?;

        query!(
            "INSERT INTO contents_people_roles (content_id, person_id, role_id) VALUES (?, ?, ?)",
            new_content_id,
            person_id,
            role_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            RitmoErr::DatabaseInsertFailed(format!("Failed to insert into contents_people_roles: {}", e))
        })?;
    }

    tx.commit()
        .await
        .map_err(|e| RitmoErr::TransactionCommitFailed(e.to_string()))?;

    Ok(new_content_id.try_into().unwrap())
}