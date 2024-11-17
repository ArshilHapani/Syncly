use clap::ArgMatches;
use color_print::cprintln;
use serde_json::{json, to_string_pretty};
use std::{fs::File, io::Write};

const FILE_NAME: &str = ".syncly.json";
const SNAPSHOT_FILE_NAME: &str = ".syncly.snapshot";

pub fn run(args: &ArgMatches) {
    cprintln!("<strong><g>Initializing Syncly in the current directory.</g></strong>");
    let override_config = args.get_flag("force");

    let json_data = json!({
        "exclude":[".git","node_modules"],
        "snapshot_ctn": 0,
    });
    let json_string = to_string_pretty(&json_data).unwrap_or("".to_string());

    if File::open(FILE_NAME).is_ok() || File::open(SNAPSHOT_FILE_NAME).is_ok() && !override_config {
        cprintln!("<strong><r>Configuration file already exists. To override your current config use '-f' or '--force' flag</r></strong>");
    } else if override_config {
        cprintln!("<strong><yellow>Overriding default config</yellow></strong>");
        let mut f = File::create(FILE_NAME).expect("Failed to create configuration file.");
        match f.write(json_string.as_bytes()) {
            Ok(_) => cprintln!("<strong><g>Configration reseted</g></strong>"),
            Err(_) => cprintln!("<strong><r>Failed to reset config</r></strong>"),
        }
    } else {
        let mut f = File::create(FILE_NAME).expect("Failed to create configuration file.");
        match f.write(json_string.as_bytes()) {
            Ok(_) => cprintln!(
                "<strong><green>Configuration file created successfully.</green></strong>"
            ),
            Err(_) => cprintln!("<strong><r>Failed to reset config</r></strong>"),
        }
    }
}
