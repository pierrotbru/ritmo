use sea_orm::{
    Condition, 
    ColumnTrait,
};
use crate::db::entity::original_languages::Column;

pub fn or_lang_filter_condition(
    name: Option<&str>,
    code: Option<&str>,
) -> Option<Condition> {
    let mut condition = Condition::all();

    // Add name filter (case-insensitive partial match)
    if let Some(name_filter) = name {
        condition = condition.add(Column::Name.contains(name_filter));
    }

    // Add code filter (exact match)
    if let Some(nat) = code {
        condition = condition.add(Column::Code.eq(nat));
    }

    if condition == Condition::all() {
        None
    }
    else {
        Some(condition)
    }
}

