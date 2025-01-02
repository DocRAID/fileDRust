use std::path::Path;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;

enum ActionDetermine {
    CreateFile,
    CreateDir,

    RemoveFile,
    RemoveDir,

    ModifyFile,
    ModifyDir,
    ModifyName
}
pub fn listner(path: &str) -> Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);
                //todo ActionDetermine.
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}