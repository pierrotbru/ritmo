use crate::db::search::query_build::ChainRung::Roof;
use crate::errors::RitmoErr;
use crate::db::search::get_struct::get_struct_table;

#[derive(Default, Debug, Clone)]
pub enum ChainRung {
	#[default] Undefined, 
	Roof,
	Mid,
	Floor
}

#[derive( Default, Debug, Clone)]
pub struct QueryLadder {
	level: ChainRung,
	table_name: String,
	column_name: Option<String>,
	join_table: Option<String>,
	join_column: Option<String>,
}

//pub async fn query_build() -> Result<Vec<TableInfo>, RitmoErr>{
pub async fn query_build() -> Result<(), RitmoErr>{

	let tables = get_struct_table().await?;
	let mut ladders = Vec::<QueryLadder>::new();

	let mut ladder = QueryLadder::default();

	ladder.level = Roof;
	ladder.table_name = "books".to_string();
	ladders.push(ladder.clone());
	for table in tables {
		if table.name == ladder.table_name {
			println!("{:?}", table);	
		}
	}
	Ok(())
}