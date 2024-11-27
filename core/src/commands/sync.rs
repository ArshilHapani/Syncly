use std::{io::Error, path::PathBuf};

use clap::ArgMatches;

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    let empty_string = String::from("");
    let source_path_str: &String = args.get_one("source").unwrap_or(&empty_string);
    let dest_path_str: &String = args.get_one("target").unwrap_or(&empty_string);

    let source_path = PathBuf::from(source_path_str);
    let dest_path = PathBuf::from(dest_path_str);
    if !source_path.is_dir()
        || !source_path.is_file()
        || !dest_path.is_dir()
        || !dest_path.is_file()
    {}
    println!("source {:}\ndest {:}", source_path_str, dest_path_str);
    Ok(())
}
