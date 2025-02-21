use crate::db::entity::prelude::*;
use crate::db::entity::*;
use sea_orm::{
    SqlxSqliteConnector, 
    EntityTrait, 
    QueryFilter, 
    ColumnTrait,
    ActiveModelTrait,
    ActiveValue
};
use sqlx::SqlitePool;
use crate::RitmoErr;
use crate::db::entity::roles;

#[async_trait::async_trait]
trait SearchAndAdd {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr>;
}

#[async_trait::async_trait]
impl SearchAndAdd for roles::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_role = Roles::find()
            .filter(roles::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If role exists, return its ID
        if let Some(role) = existing_role {
            return Ok(role.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_role = roles::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_role = new_role
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_role.id);
        }
        
        // If no role exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Role '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for people::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_people = People::find()
            .filter(people::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If people exists, return its ID
        if let Some(people) = existing_people {
            return Ok(people.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_people = people::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_people = new_people
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_people.id);
        }
        
        // If no role exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("People '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for tags::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_tags = Tags::find()
            .filter(tags::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If tags exists, return its ID
        if let Some(tags) = existing_tags {
            return Ok(tags.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_tags = tags::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_tags = new_tags
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_tags.id);
        }
        
        // If no role exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Tags '{}' not found", target)))
    }
}
