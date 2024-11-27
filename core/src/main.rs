mod commands;
mod config;
mod display;
mod state;

use commands::{build_cli, initialize, reset, sync as sync_cmd};
use display::show_menu;

fn main() -> Result<(), std::io::Error> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", flg)) => initialize::run(flg)?,
        Some(("reset", _)) => reset::run()?,
        Some(("sync", args)) => sync_cmd::run(args)?,
        _ => {
            show_menu();
            std::process::exit(1);
        }
    }
    Ok(())
}
