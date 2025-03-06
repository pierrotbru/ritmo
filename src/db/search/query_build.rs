use crate::errors::RitmoErr;
use crate::db::search::get_struct::*;

#[derive(Default, Debug, Clone)]
pub enum ChainRung {
	#[default] Roof,
	Mid,
	Floor,
}

#[derive(Default, Debug, Clone)]
pub struct QueryRung {
    level: ChainRung,
    table_name: String,
    column_name: Option<String>,
    join_table: Option<String>,
    join_column: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct QueryLadder {
    name: String,
    rungs: Vec<QueryRung>,
}

pub async fn query_build(item: String) -> Result<Vec<QueryLadder>, RitmoErr> {
    let tables = get_struct_table().await?;
    let mut ladders = Vec::<QueryLadder>::new();

  
  // select the table in tables with the 'item' name
  let table = tables
      .into_iter()
      .find(|table| table.name == item);

    // handle errors
    let table = match table {
        Some(t) => t,
        None => return Err(RitmoErr::RecordNotFound),
    };

    let mut ladders = Vec::<QueryLadder>::new();

    for (col_name, _) in table.column_info.iter() {
        let mut ladder = QueryLadder::default();
        let mut rung = QueryRung::default();

        ladder.name = col_name.clone();
        rung.level = ChainRung::Floor;
        rung.table_name = table.name.clone();
        rung.column_name = Some(col_name.clone());

        ladder.rungs.push(rung);
        ladders.push(ladder);
    }

    for (a,b,c) in table.foreign_keys {
    	
    }

    Ok(ladders)
}
