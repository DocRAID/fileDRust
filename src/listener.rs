use std::path::Path;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::Sender;
use crate::configure::SourceConfig;

enum ActionDetermine {
    CreateFile,
    CreateDir,

    Remove,

    ModifyFile,

    RenameDir,
    RenameFile,

    Unknown
}
pub fn listner(path: &str, config:Arc<SourceConfig>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);

                let is_temp_file = event.paths.last()
                    .and_then(|path| path.file_name())
                    .map(|name| name.as_encoded_bytes().starts_with(b"~") || name.as_encoded_bytes().ends_with(b"~"))
                    .expect("No file name found.");

                if !(!config.reflect_temporary_file && is_temp_file) {  //임시파일 적용 여부 확인
                    match event.kind {
                        EventKind::Create(_) => {

                        }
                        EventKind::Modify(_) => {

                        }
                        EventKind::Remove(_) => {

                        }
                        _=>{}
                        // EventKind::Access(_) => {}
                        // EventKind::Any => {}
                        // EventKind::Other => {}
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

