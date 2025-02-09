use sea_orm::{
    Condition, 
    ColumnTrait,
};
use crate::db::entity::roles::Column;

pub fn roles_filter_condition(
    name: &str,
) -> Option<Condition> {
    let mut condition = Condition::all();
    condition = condition.add(Column::Name.contains(name));
    if condition == Condition::all() {
        None
    }
    else {
        Some(condition)
    }
}
