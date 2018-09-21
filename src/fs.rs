use std::path::PathBuf;

use xattr;
use walkdir::WalkDir;

pub fn find_paths(root: &str, attr_filter: &str) -> Vec<PathBuf> {
    let mut backup_dirs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(root).max_depth(2) {
        match entry {
            Ok(e) => match e.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if let Ok(Some(attr)) = xattr::get(e.path(), "user.backup") {
                            if String::from_utf8_lossy(&attr) == attr_filter {
                                backup_dirs.push(e.path().into());
                            }
                        }
                    }
                }
                Err(em) => {
                    eprintln!(
                        "{}: failed to retrieve metadata ({:?})",
                        e.path().to_string_lossy(),
                        em
                    );
                }
            },
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    backup_dirs
}

