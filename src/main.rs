#[macro_use]
extern crate failure;
extern crate clap;
extern crate walkdir;
extern crate xattr;

use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use clap::{Arg, App, SubCommand};
use walkdir::WalkDir;

fn find_paths(root: &str) -> Vec<PathBuf> {
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

    backup_dirs
}

fn process_duplicity(root: &str, key: &str, target: &str, force_full: bool) {
    let backup_dirs = find_paths(root);

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

fn process_list(root: &str) {
    for path in find_paths(root) {
        println!("{}", OsString::from(path).to_string_lossy());
    }
}

#[derive(Debug, Fail)]
enum BackupWrapperError {
    #[fail(display = "The backup root parameter is required but it was not provided")]
    BackupRootMissing,
    #[fail(display = "The backup target parameter is required but it was not provided")]
    BackupTargetMissing,
    #[fail(display = "Unknown command, see --help")]
    UnknownCommand,
}

fn main() -> Result<(), BackupWrapperError> {
    let app_matches = App::new("backup-wrapper")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("root")
             .short("r")
             .long("root")
             .value_name("DIRECTORY")
             .help("Set root directory for backup")
             .takes_value(true))
        .arg(Arg::with_name("target")
             .short("t")
             .long("target")
             .value_name("TARGET")
             .help("Set backup target")
             .takes_value(true))
        .subcommand(SubCommand::with_name("duplicity")
                    .about("Performs a backup using the duplicity tool")
                    .arg(Arg::with_name("key")
                         .short("k")
                         .long("key")
                         .value_name("KEY")
                         .help("Encryption key fingerprint")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("full")
                         .short("f")
                         .long("full")
                         .help("Force full backup")))
        .subcommand(SubCommand::with_name("restic")
                    .about("Performs a backup using the restic tool"))
        .subcommand(SubCommand::with_name("list")
                    .about("Show the list of directories to be included in the backup"))
        .get_matches();

    if let Some(matches) = app_matches.subcommand_matches("duplicity") {
        if let Some(root) = app_matches.value_of("root") {
            if let Some(target) = app_matches.value_of("target") {
                process_duplicity(
                    root,
                    matches.value_of("key").unwrap(),
                    target,
                    matches.is_present("full")
                );

                Ok(())
            } else {
                Err(BackupWrapperError::BackupTargetMissing)
            }
        } else {
            Err(BackupWrapperError::BackupRootMissing)
        }
    } else if let Some(_matches) = app_matches.subcommand_matches("restic") {
        unimplemented!()
    } else if let Some(_matches) = app_matches.subcommand_matches("list") {
        if let Some(root) = app_matches.value_of("root") {
            process_list(root);

            Ok(())
        } else {
            Err(BackupWrapperError::BackupRootMissing)
        }
    } else {
        Err(BackupWrapperError::UnknownCommand)
    }
}
