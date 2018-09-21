use fs::find_paths;

use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn process(root: &str, key: &str, target: &str, mode: &str) {
    match mode {
        "clean" => {
            Command::new("duplicity")
                .arg("remove-all-but-n-full")
                .arg("2")
                .arg(target)
                .exec();
        },
        "incremental" | "full" => {
            let backup_dirs = find_paths(root);

            Command::new("duplicity")
                .arg(mode)
                .arg("-v4")
                .arg("--archive-dir")
                .arg("/var/backups/duplicity")
                .arg("--use-agent")
                .arg("--encrypt-sign-key")
                .arg(key)
                .args(
                    backup_dirs
                        .into_iter()
                        .flat_map(|p| vec![OsString::from("--include"), OsString::from(p)]),
                ).arg("--exclude")
                .arg("**")
                .arg(root)
                .arg(target)
                .exec();
        }
        _ => panic!(format!("Invalid mode: {}", mode)),
    }
}
