use crate::RitmoErr;
use crate::PathBuf;

pub fn verify_path(path: &PathBuf, create: bool) -> Result<PathBuf, RitmoErr> {

    // Canonicalize the path to resolve symbolic links and ".." components
    let mut out_path = path.clone();

    if out_path.is_relative() {
        out_path = std::env::current_dir()
            .map_err(|e| RitmoErr::PathError(format!("Failed to get current directory: {}", e)))?
            .join(&out_path);
    }

    out_path = match out_path.canonicalize() {
        Ok(_) => {
            if create == true {
                return Err(RitmoErr::PathError(format!("Path is not valid. Directory {} already exists.", out_path.display())));
            }
            else {
                if out_path.is_dir() {
                    out_path = out_path.join("ritmo.db");
                }
                out_path
            }
        }
        Err(e) => {
            if create && e.kind() == std::io::ErrorKind::NotFound {
                // If create is true and the file doesn't exist, that's okay
                // create the full filename path adding the filename "ritmo.db"
                out_path.push("ritmo.db");
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
