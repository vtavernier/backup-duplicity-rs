use std::path::PathBuf;

use xattr;
use walkdir::WalkDir;

pub fn find_paths(root: &str) -> Vec<PathBuf> {
    let mut backup_dirs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(root).max_depth(2) {
        match entry {
            Ok(e) => match e.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if let Ok(Some(attr)) = xattr::get(e.path(), "user.backup") {
                            if attr == b"1" {
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

