use crate::errors::RitmoErr;
use sqlx::sqlite::SqlitePool;

pub async fn copy_data_from_calibre_db(
    calibre_conn : &SqlitePool, 
    my_conn: &SqlitePool
) -> Result<(), RitmoErr> {

    Ok(())
}
