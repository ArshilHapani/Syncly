mod utils;

use std::env::args;

use utils::show_menu;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        show_menu();
    }
}
