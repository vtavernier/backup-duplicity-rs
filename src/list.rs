use ::fs::find_paths;

use std::ffi::OsString;

pub fn process(root: &str) {
    for path in find_paths(root) {
        println!("{}", OsString::from(path).to_string_lossy());
    }
}
