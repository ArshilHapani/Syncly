use std::path::PathBuf;

use clap::ArgMatches;
use snapshots::watcher::watch_directory;

use crate::state::SynclyErrorKind;

pub fn run(args: &ArgMatches) -> Result<(), SynclyErrorKind> {
    let empty_string = String::from("");
    let source_path_str: &String = args.get_one("source").unwrap_or(&empty_string);
    let dest_path_str: &String = args.get_one(r"target").unwrap_or(&empty_string);

    let source_path = PathBuf::from(source_path_str);
    let dest_path = PathBuf::from(dest_path_str);
    let is_source_exist = source_path.is_dir() || source_path.is_file();
    let is_dest_exist = dest_path.is_dir() || dest_path.is_file();

    if !is_source_exist {
        return Err(SynclyErrorKind::SourceFileOrDirNotFound);
    }

    if !is_dest_exist {
        return Err(SynclyErrorKind::DestFileOrDirNotFound);
    }
    let _ = watch_directory(&source_path);
    println!("source {:?}\ndest {:?}", source_path, dest_path);

    Ok(())
}
