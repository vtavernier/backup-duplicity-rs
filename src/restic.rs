use fs::find_paths;

use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn process(level: &str, root: &str, password_file: &str, mode: &str) {
    match mode {
        "clean" => {
            Command::new("restic")
                .arg("-p")
                .arg(password_file)
                .arg("forget")
                .arg("--keep-daily")
                .arg("7")
                .arg("--keep-weekly")
                .arg("2")
                .arg("--prune")
                .exec();
        },
        "backup" => {
            let backup_dirs = find_paths(root, level);

            Command::new("restic")
                .arg("-p")
                .arg(password_file)
                .arg(mode)
                .args(backup_dirs.into_iter())
                .exec();
        }
        _ => panic!(format!("Invalid mode: {}", mode)),
    }
}
