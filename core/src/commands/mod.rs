pub mod initialize;
pub mod reset;
pub mod sync;

use clap::{Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new("syncly")
        .display_name("Syncly")
        .version("1.0")
        .about("Effortlessly synchronize files and directories across devices.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        //////////////////////////////////////////////////////////////////////
        //////////////////////////////// INIT ////////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(
            Command::new("init")
                .about("Initialize Syncly in the current directory.")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force initialization by overwriting existing configuration.")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        //////////////////////////////////////////////////////////////////////
        //////////////////////////////// SYNC ////////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(
            Command::new("sync")
                .about("Synchronize files between source and target directories.")
                .arg(
                    Arg::new("source")
                        .short('s')
                        .long("source")
                        .value_name("PATH")
                        .required(true)
                        .help("Specify the source directory for synchronization."),
                )
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("PATH")
                        .required(true)
                        .help("Specify the target directory for synchronization."),
                )
                .arg(
                    Arg::new("exclude")
                        .short('e')
                        .long("exclude")
                        .value_name("LIST")
                        .help("Exclude specific files or directories (comma-separated)."),
                )
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .help("Synchronize directories recursively."),
                )
                .arg(
                    Arg::new("dry-run")
                        .short('d')
                        .long("dry-run")
                        .help("Simulate synchronization without making changes."),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Enable verbose output for detailed logs."),
                ),
        )
        //////////////////////////////////////////////////////////////////////
        /////////////////////////////// STATUS ///////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(
            Command::new("status").about("Show the synchronization status and pending changes."),
        )
        //////////////////////////////////////////////////////////////////////
        /////////////////////////////// CONFIG ///////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(
            Command::new("config")
                .about("Configure Syncly settings.")
                .arg(
                    Arg::new("exclude")
                        .short('e')
                        .long("exclude")
                        .value_name("LIST")
                        .help("Exclude specific files or directories (comma-separated)."),
                ),
        )
        //////////////////////////////////////////////////////////////////////
        ////////////////////////////// HISTORY ///////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(Command::new("history").about("View the synchronization history."))
        //////////////////////////////////////////////////////////////////////
        /////////////////////////////// RESET ////////////////////////////////
        //////////////////////////////////////////////////////////////////////
        .subcommand(Command::new("reset").about("Reset the synchronization state."))
}
