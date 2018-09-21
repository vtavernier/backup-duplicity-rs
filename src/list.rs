use ::fs::find_paths;

use std::ffi::OsString;

pub fn process(level: &str, root: &str) {
    for path in find_paths(root, level) {
        println!("{}", OsString::from(path).to_string_lossy());
    }
}
