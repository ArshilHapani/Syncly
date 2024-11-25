use std::path::PathBuf;

use color_print::cprintln;
use serde_json::to_string_pretty;
use snapshots::utils::modify_file_content;

use crate::config::{get_default_config, CONFIG_FILE_NAME, SNAPSHOT_FILE_NAME};

pub fn run() -> Result<(), std::io::Error> {
    let json_data = get_default_config();
    let json_string = to_string_pretty(&json_data).unwrap_or("".to_string());

    cprintln!("<strong><yellow>Overriding current config</yellow></strong>");

    let config_file_path = PathBuf::from(CONFIG_FILE_NAME);
    let snapshot_file_path = PathBuf::from(SNAPSHOT_FILE_NAME);

    modify_file_content(&config_file_path, json_string.as_bytes(), true)?;
    modify_file_content(&snapshot_file_path, "".as_bytes(), true)?;

    cprintln!("<strong><g>Configration reseted</g></strong>");
    Ok(())
}
