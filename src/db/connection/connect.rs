use crate::errors::RitmoErr;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::path::PathBuf;

pub async fn connect_to_db(db: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    let fname = verify_path(db, create)?;

    let options = SqliteConnectOptions::new()
        .filename(&fname)
        .create_if_missing(create);

    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

fn verify_path(path: &PathBuf, create: bool) -> Result<PathBuf, RitmoErr> {

    // Canonicalize the path to resolve symbolic links and ".." components
    let mut out_path = path.clone();

    out_path = match out_path.canonicalize() {
        Ok(_) => {
            if create == true {
                return Err(RitmoErr::PathError(format!("Path is not valid. Directory {} already exists.", out_path.display())));
            }
            else {
                if out_path.is_dir() {
                    out_path = out_path.join("dbbooks.db");
                }
                out_path
            }
        }
        Err(e) => {
            if create && e.kind() == std::io::ErrorKind::NotFound {
                // If create is true and the file doesn't exist, that's okay
                // create the full filename path adding the filename "dbbooks.db"
                out_path.push("dbbooks.db");
                // create the parent directory
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| RitmoErr::PathError(format!("Failed to create parent directory: {}", e)))?;
                }
                out_path
            } else {
                return Err(RitmoErr::PathError(format!("Path is not valid: {}", e)));
            }
        }
    };

    Ok(out_path)
}
