use sea_orm::{
    Condition, 
    ColumnTrait,
};
use crate::db::entity::aliases::Column;

pub fn people_filter_condition(
    name: &str,
) -> Option<Condition> {
    let mut condition = Condition::all();
    condition = condition.add(Column::Alias.contains(name));
    if condition == Condition::all() {
        None
    }
    else {
        Some(condition)
    }
}
