use clap::ArgMatches;
use color_print::cprintln;
use serde_json::to_string_pretty;
use std::{fs::File, io::Write};

use crate::{
    config::{get_default_config, CONFIG_FILE_NAME, SNAPSHOT_FILE_NAME},
    state::SynclyErrorKind,
};

pub fn run(args: &ArgMatches) -> Result<(), SynclyErrorKind> {
    let override_config = args.get_flag("force");

    let json_data = get_default_config();
    let json_string = to_string_pretty(&json_data).unwrap_or("".to_string());

    if !override_config
        && (File::open(CONFIG_FILE_NAME).is_ok() || File::open(SNAPSHOT_FILE_NAME).is_ok())
    {
        cprintln!("<strong><r>Configuration file already exists. To override your current config use '-f' or '--force' flag</r></strong>");
        Ok(())
    } else if override_config {
        // reset the configuration files
        crate::reset::run()
    } else {
        cprintln!(
            "<strong><yellow>Initializing Syncly in the current directory.</yellow></strong>"
        );
        // Create the configuration and snapshot files
        File::create(SNAPSHOT_FILE_NAME)?;
        cprintln!("<strong><green>Snapshot file created successfully.</green></strong>");
        let mut file = File::create(CONFIG_FILE_NAME)?;
        file.write(json_string.as_bytes())?;
        file.sync_all().map_err(|e| SynclyErrorKind::SyncError(e))?;
        file.flush().map_err(|e| SynclyErrorKind::FlushError(e))?;
        cprintln!("<strong><green>Configuration file created successfully.</green></strong>");
        Ok(())
    }
}
