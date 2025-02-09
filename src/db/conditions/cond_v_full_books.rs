use sea_orm::{
    sea_query::{Condition, SimpleExpr},
    ColumnTrait, EntityTrait, QueryFilter,
};

use super::super::entity::v_full_books::Entity;

pub struct VFullBooksConditions;

impl VFullBooksConditions {
    /// Filter by book title (case-insensitive, partial match)
    pub fn by_book_title(title: &str) -> Condition {
        Condition::all().add(
            Entity::Column::BookTitle.contains(title)
        )
    }

    /// Filter by book original title (case-insensitive, partial match)
    pub fn by_book_original_title(original_title: &str) -> Condition {
        Condition::all().add(
            Entity::Column::BookOriginalTitle.contains(original_title)
        )
    }

    /// Filter by publication date range
    pub fn by_publication_date_range(start_date: Option<&str>, end_date: Option<&str>) -> Condition {
        let mut condition = Condition::all();
        
        if let Some(start) = start_date {
            condition = condition.add(Entity::Column::PublicationDate.gt(start));
        }
        
        if let Some(end) = end_date {
            condition = condition.add(Entity::Column::PublicationDate.lt(end));
        }
        
        condition
    }

    /// Filter by series name
    pub fn by_series_name(series_name: &str) -> Condition {
        Condition::all().add(
            Entity::Column::SeriesName.contains(series_name)
        )
    }

    /// Filter by book tags
    pub fn by_book_tags(tag: &str) -> Condition {
        Condition::all().add(
            Entity::Column::BookTags.contains(tag)
        )
    }

    /// Filter by book people (authors, editors, etc.)
    pub fn by_book_people(person_name: &str) -> Condition {
        Condition::all().add(
            Entity::Column::BookPeople.contains(person_name)
        )
    }

    /// Filter by content type
    pub fn by_content_type(content_type: &str) -> Condition {
        Condition::all().add(
            Entity::Column::ContentTypes.contains(content_type)
        )
    }

    /// Filter by language
    pub fn by_language(language_code: &str) -> Condition {
        Condition::all().add(
            Condition::any()
                .add(Entity::Column::ContentCurrentLanguages.contains(language_code))
                .add(Entity::Column::ContentOriginalLanguages.contains(language_code))
                .add(Entity::Column::ContentSourceLanguages.contains(language_code))
        )
    }

    /// Combine multiple conditions
    pub fn combine_conditions(conditions: Vec<Condition>) -> Condition {
        conditions.into_iter().fold(Condition::all(), |acc, condition| acc.add(condition))
    }
}