mod commands;
mod config;
mod display;
mod state;

use commands::{build_cli, initialize};
use display::show_menu;

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", flg)) => initialize::run(flg),
        _ => {
            show_menu();
            std::process::exit(1);
        }
    }
}
