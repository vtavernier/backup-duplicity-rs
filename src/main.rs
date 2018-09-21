#[macro_use]
extern crate failure;
extern crate clap;
extern crate walkdir;
extern crate xattr;

use clap::{Arg, App, SubCommand};

mod fs;
mod duplicity;
mod list;

#[derive(Debug, Fail)]
enum BackupWrapperError {
    #[fail(display = "Unknown command, see --help")]
    UnknownCommand,
}

fn main() -> Result<(), BackupWrapperError> {
    let app_matches = App::new("backup-wrapper")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("duplicity")
                    .about("Performs a backup using the duplicity tool")
                    .arg(Arg::with_name("root")
                         .short("r")
                         .long("root")
                         .value_name("DIRECTORY")
                         .help("Set root directory for backup")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("target")
                         .short("t")
                         .long("target")
                         .value_name("TARGET")
                         .help("Set duplicity backup target URL")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("key")
                         .short("k")
                         .long("key")
                         .value_name("KEY")
                         .help("Encryption key fingerprint")
                         .takes_value(true)
                         .required(true))
                    .subcommand(SubCommand::with_name("full")
                                .about("Perform a full backup"))
                    .subcommand(SubCommand::with_name("incremental")
                                .about("Perform an incremental backup"))
                    .subcommand(SubCommand::with_name("clean")
                                .about("Clean old backups")))
        .subcommand(SubCommand::with_name("restic")
                    .about("Performs a backup using the restic tool"))
        .subcommand(SubCommand::with_name("list")
                    .about("Show the list of directories to be included in the backup")
                    .arg(Arg::with_name("root")
                         .short("r")
                         .long("root")
                         .value_name("DIRECTORY")
                         .help("Set root directory for search")
                         .takes_value(true)
                         .required(true)))
        .get_matches();

    if let Some(matches) = app_matches.subcommand_matches("duplicity") {
        let root = matches.value_of("root").unwrap();
        let key = matches.value_of("key").unwrap();
        let target = matches.value_of("target").unwrap();
        let mode = if let Some(_matches) = matches.subcommand_matches("full") {
            Some("full")
        } else if let Some(_matches) = matches.subcommand_matches("incremental") {
            Some("incremental")
        } else if let Some(_matches) = matches.subcommand_matches("clean") {
            Some("clean")
        } else {
            None
        };

        if let Some(mode) = mode {
            duplicity::process(root, key, target, mode);

            Ok(())
        } else {
            Err(BackupWrapperError::UnknownCommand)
        }
    } else if let Some(_matches) = app_matches.subcommand_matches("restic") {
        unimplemented!()
    } else if let Some(matches) = app_matches.subcommand_matches("list") {
        list::process(matches.value_of("root").unwrap());

        Ok(())
    } else {
        Err(BackupWrapperError::UnknownCommand)
    }
}
