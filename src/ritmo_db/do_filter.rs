use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;

use crate::ritmo_db::entities::{prelude::*, *};

pub async fn get_book_ids_by_person_name(conn: &DatabaseConnection, person_name: &str) -> Result<Vec<i32>, DbErr> {
//    let person = people::Entity::find()
//        .filter(people::Column::Name.like(format!("%{}%",person_name)))
//        .one(conn)
//        .await?;

    let person = match people::Entity::find()
        .filter(people::Column::Name.like(format!("%{}%", person_name)))
        .one(conn)
        .await
    {
        Ok(person) => person,
        Err(err) => {
            return Err(err); // Propaga l'errore
        }
    };

    if let Some(person) = person {
        let contents_relations = contents_people::Entity::find()
            .filter(contents_people::Column::PersonId.eq(person.id))
            .find_also_related(contents::Entity)
            .all(conn)
            .await?;

        let mut book_ids = Vec::new();
        for (_, content_opt) in contents_relations {
            if let Some(content) = content_opt { // Gestisci l'Option<contents::Model>
                book_ids.extend(
                    books_contents::Entity::find()
                        .filter(books_contents::Column::ContentId.eq(content.id))
                        .all(conn)
                        .await?
                        .into_iter()
                        .map(|bc| bc.book_id),
                );
            }
        }

        Ok(book_ids)
    } else {
        Ok(Vec::new())
    }
}