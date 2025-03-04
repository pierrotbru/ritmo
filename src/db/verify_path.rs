use crate::RitmoErr;
use crate::PathBuf;
use std::fs;
use std::io;

// Create a new database if requested and the directory does not exist
pub fn verify_path(path: &PathBuf, create: bool) -> Result<PathBuf, RitmoErr> {
    // Canonicalize the path to resolve symbolic links and ".." components
    let mut out_path = path.clone();

    if out_path.is_relative() {
        out_path = std::env::current_dir()
            .map_err(|e| RitmoErr::PathError(format!("Failed to get current path: {}", e)))?
            .join(&out_path);
    }

    // Attempt to canonicalize the path
    match out_path.canonicalize() {
        Ok(_) => {
            if create {
                return Err(RitmoErr::PathError(format!("Path is not valid. Directory {} already exists.", out_path.display())));
            } else if out_path.is_dir() {
                out_path.push("ritmo.db");
            }
        }
        Err(e) => {
            if create && e.kind() == io::ErrorKind::NotFound {
                out_path.push("ritmo.db");
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| RitmoErr::PathError(format!("Failed to create parent directory: {}", e)))?;
                }
            } else {
                return Err(RitmoErr::PathError(format!("Path is not valid: {}", e)));
            }
        }
    };

    Ok(out_path)
}
