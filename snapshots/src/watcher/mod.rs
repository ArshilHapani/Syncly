use notify::{recommended_watcher, Error, RecursiveMode, Watcher};
use std::{path::PathBuf, sync::mpsc::channel};

pub fn watch_directory(dir: &PathBuf) -> Result<(), Error> {
    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(tx)?;
    loop {
        watcher.watch(dir, RecursiveMode::Recursive)?;
        let event = rx.recv()?;
        println!("Changes detected {:?}", event);
    }

    // match recommended_watcher(tx) {
    //     Ok(mut watcher) => loop {
    //         match watcher.watch(dir, RecursiveMode::Recursive) {
    //             Ok(_) => match rx.recv() {
    //                 Ok(event) => println!("Changes detected {:?}", event),
    //                 Err(e) => eprintln!("watch error {:?}", e),
    //             },
    //             Err(e) => return Err(e.into()),
    //         };
    //     },
    //     Err(e) => return Err(e.into()),
    // }
}
