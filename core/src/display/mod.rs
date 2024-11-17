use color_print::cprintln;

pub fn show_menu() {
    cprintln!("<strong><b>Syncly - Fast Synchronizer CLI</b></strong>");
    cprintln!("------------------------------");
    cprintln!("<strong>Effortlessly synchronize files and directories</strong>");
    println!();
    cprintln!("<strong><b>Usage:</b>\n    syncly [command] [options]</strong>");
    println!();

    cprintln!("<strong><b>Commands:</b></strong>");
    let syncly_json_string = colors::secondary_cyan(".syncly.json");
    cprintln!("<strong> <yellow>   init </yellow>                  Initialize the project in current directory and creates {} file.</strong>",syncly_json_string);
    cprintln!("<strong>    <yellow>sync [SOURCE] [DEST]</yellow>   Starts syncing files and folders from source directory to current directory.</strong>");
    cprintln!(
        "<strong>    <yellow>status </yellow>                Shows the syncing status of current job.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>config </yellow>                Shows the current configuration.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>history </yellow>               Shows the previous snapshots.</strong>"
    );
    cprintln!("<strong>    <yellow>reset </yellow>                 Reset the synchronization state.</strong>");
    cprintln!("<strong>    <yellow>help </yellow>                  Shows this menu.</strong>");
    println!();

    cprintln!("<strong><b>Options:</b></strong>");
    cprintln!(
        "<strong>    <yellow>-e, --exclude [LIST]</yellow> List of file names to exclude.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>-p, --pattern [LIST]</yellow> List of file extension patterns to exclude.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>-r, --recursive</yellow>      Synchronize directory recursively.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>-d, --dry-run</yellow>        Simulate synchronization without making changes.</strong>"
    );
    cprintln!(
        "<strong>    <yellow>-v, --verbose</yellow>        Show detailed information during synchronization.</strong>"
    );
    cprintln!("<strong>    <yellow>-h, --help</yellow>           Shows this menu.</strong>");
    println!();

    cprintln!("<strong><b>Examples:</b></strong>");

    cprintln!("<strong> <yellow>    syncly init</yellow> </strong>");
    cprintln!("<strong>         Initialize the empty project in current directory.</strong>");
    println!();
    cprintln!("<strong> <yellow>    syncly sync ./client/prisma/schema.prisma ./admin/prisma/schema.prisma</yellow></strong>");
    cprintln!(
        "<strong>         Syncs the schema.prisma file from client to admin directory.</strong>"
    );
    println!();
    cprintln!("<strong> <yellow>    syncly status </yellow>");
    cprintln!(
        "<strong>         Shows the pending state and current synchronization status.</strong>"
    );
    println!();
    cprintln!("<strong> <yellow>    syncly config --exclude node_modules, cache, .git</yellow>");
    cprintln!("<strong>         Configuration exclusion for synchronization.</strong>");
    println!();
    cprintln!("<strong> <yellow>    syncly reset</yellow>");
    cprintln!("<strong>         Resets the synchronization state and clears the history.</strong>");
}

mod colors {
    use color_print::cformat;
    pub fn secondary_cyan(st: &str) -> String {
        cformat!("<rgb(144,224,239)>{}</rgb(144,224,239)>", st)
    }
}
