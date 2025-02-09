use sea_orm::{
    Condition, 
    ColumnTrait,
};
use crate::db::entity::people::Column;

pub fn people_filter_condition(
    name: Option<&str>,
    nationality: Option<&str>,
    birth_year: Option<&str>,
) -> Option<Condition> {
    let mut condition = Condition::all();

    // Add name filter (case-insensitive partial match)
    if let Some(name_filter) = name {
        condition = condition.add(Column::Name.contains(name_filter));
    }

    // Add nationality filter (exact match)
    if let Some(nat) = nationality {
        condition = condition.add(Column::Nationality.eq(nat));
    }

    // Add birth year filter (exact match)
    if let Some(year) = birth_year {
        condition = condition.add(Column::BirthYear.eq(year));
    }
    if condition == Condition::all() {
        None
    }
    else {
        Some(condition)
    }
}

// Example usage:
// let query = Entity::find()
//     .filter(
//         people_filter_condition(Some("John"), None, Some("1980"))
//         .add(condition1)
//         .add(condition2)
//         .add(condition3)
//     )
//     .all(db)
//     .await?;

