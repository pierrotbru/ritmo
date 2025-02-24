use crate::db::entity::contents_types;
use sea_orm::ActiveValue;
use crate::RitmoErr;
use sqlx::SqlitePool;
use crate::db::adds::search_and_add::SearchAndAdd;

pub struct ContentData {
	pub name: String,
    pub original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub notes: Option<String>,
    pub type_id: Option<String>,
    pub pre_accepted: Option<i32>,
    pub src_lang: Option<String>,
    pub curr_lang: Option<String>,
    pub orig_lang: Option<String>,
    pub people: Vec<(String,String)>,
    pub tags: Option<String>
}

pub async fn add_content(pool: SqlitePool, content: &ContentData) -> Result<i32, RitmoErr> {

	let mut new_content = crate::db::entity::contents::ActiveModel {
		name: ActiveValue::Set(content.name.clone()),
		original_title: ActiveValue::Set(content.original_title.clone()),
		publication_date: ActiveValue::Set(content.publication_date.clone()),
		..Default::default()
	};

    let type_id_str = match &content.type_id {
        Some(s) => s.as_str(),
        None => return Err(RitmoErr::NoResultsError("Type ID cannot be None".to_string())), // Gestisci il caso None
    };
	let ct = contents_types::Model {id: 1, name: "".to_string()};
    let n = ct.search_and_add(&pool, type_id_str, true).await?;
	new_content.type_id = ActiveValue::Set(Some(n));


	Ok(0)
}
