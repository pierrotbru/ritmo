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
pub trait SearchAndAdd {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr>;
}

#[async_trait::async_trait]
impl SearchAndAdd for aliases::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(aliases::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = aliases::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for books::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(books::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = books::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for contents::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(contents::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = contents::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for contents_types::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(contents_types::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = contents_types::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for formats::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(formats::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = formats::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for people::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(people::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = people::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for publishers::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(publishers::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = publishers::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for roles::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(roles::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = roles::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for series::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(series::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = series::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}

#[async_trait::async_trait]
impl SearchAndAdd for tags::Model {
    async fn search_and_add(&self, pool: &SqlitePool, target: &str, add_it: bool) -> Result<i32, RitmoErr> {
        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        
        // First, try to find an existing role
        let existing_data = Publishers::find()
            .filter(tags::Column::Name.eq(target))
            .one(&conn)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        // If data exists, return its ID
        if let Some(data) = existing_data {
            return Ok(data.id);
        }
        
        // If add_it is true and no role exists, create a new role
        if add_it {
            let new_data = tags::ActiveModel {
                name: ActiveValue::Set(target.to_string()),
                ..Default::default()
            };
            
            let inserted_data = new_data
                .insert(&conn)
                .await
                .map_err(|e| RitmoErr::DatabaseInsertFailed(e.to_string()))?;
            
            return Ok(inserted_data.id);
        }
        
        // If no data exists and add_it is false, return an error
        Err(RitmoErr::NoResultsError(format!("Publishers '{}' not found", target)))
    }
}
