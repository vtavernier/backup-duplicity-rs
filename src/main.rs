extern crate getopts;
extern crate walkdir;
extern crate xattr;

use std::env;
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use getopts::Options;
use walkdir::WalkDir;

fn process(root: &str, key: &str, target: &str, force_full: bool) {
    let mut backup_dirs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(root).max_depth(2) {
        match entry {
            Ok(e) => match e.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if let Ok(Some(attr)) = xattr::get(e.path(), "user.backup") {
                            if attr == b"1" {
                                backup_dirs.push(e.path().into());
                                println!("{}", e.path().to_string_lossy());
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

    Command::new("duplicity")
        .arg(if force_full { "incremental" } else { "full" })
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
        )
        .arg("--exclude")
        .arg("**")
        .arg(root)
        .arg(target)
        .exec();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("r", "", "set root directory for backup", "ROOT");
    opts.optopt("k", "", "set encryption key for duplicity", "KEY");
    opts.optopt("t", "", "set target for duplicity", "TARGET");
    opts.optflag("f", "full", "force full backup");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let root = matches.opt_str("r");
    if root.is_none() {
        print_usage(&program, opts);
        return;
    }

    let key = matches.opt_str("k");
    if key.is_none() {
        print_usage(&program, opts);
        return;
    }

    let target = matches.opt_str("t");
    if target.is_none() {
        print_usage(&program, opts);
        return;
    }

    process(
        &root.unwrap(),
        &key.unwrap(),
        &target.unwrap(),
        matches.opt_present("f"),
    );
}
