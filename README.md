# backup-duplicity-rs

backup-duplicity-rs is a binary that runs duplicity with arguments corresponding
to directories that have been found under a common root, with the xattr
`user.backup = 1`.

The resulting binary can have added capabilities in order to run backups as a
standard user, while still being able to backup the whole filesystem.

```
~ # setcap cap_dac_read_search=ep path/to/backup-duplicity
~ # setcap cap_dac_read_search=i path/to/duplicity-python
```

Note that this is possible since this program simply execs the duplicity binary.
This has security implications if you cannot ensure the integrity of the
duplicity installation.

## Usage

```
~ $ backup-duplicity -f -r /backup-root -k GPG-KEY-ID -t some://backup-url
```

## License

This program is available under the terms of the [MIT License](LICENSE).

## Author

Vincent Tavernier <vince.tavernier@gmail.com>
