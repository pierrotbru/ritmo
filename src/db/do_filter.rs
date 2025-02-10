use sea_orm_migration::prelude::JoinType;
use crate::db::entity;
use crate::RitmoErr;
use sea_orm::{
    DatabaseConnection, 
    EntityTrait, 
    QueryFilter,
    ColumnTrait,
    QuerySelect,
    RelationTrait
};

pub async fn get_book_ids_by_person_name(
    db: &DatabaseConnection,
    person_name: &str,
) -> Result<Vec<i64>, RitmoErr> {
    let person = PeopleEntity::find()
        .filter(PeopleColumn::Name.contains(person_name))
        .one(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?;

    println!("100");

    let person_id = match person {
        Some(p) => p.id,
        None => return Ok(vec![]), // Gestisci il caso in cui la persona non viene trovata
    };

    println!("200");

    let book_ids: Vec<i64> = ContentsPeopleEntity::find()
        .filter(ContentsPeopleColumn::PersonId.eq(person_id))
        .join(JoinType::InnerJoin, BooksContents::Relation::Contents.def()) // Join con contents_books attraverso contents
        .select_only()
        .column(BooksContentsColumn::BookId)
        .into_tuple::<(i64,)>()
        .all(db)
        .await
        .map_err(|e| RitmoErr::DatabaseError(e.to_string()))?
        .into_iter()
        .map(|(book_id,)| BookId)
        .collect();

    Ok(book_ids)
}

