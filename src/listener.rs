use crate::configure::SourceConfig;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};

#[derive(Debug)]
enum ActionDetermine {
    CreateFile,
    CreateDir,

    Remove,

    ModifyFile,

    RenameDir,
    RenameFile,

    Unknown,
}
#[derive(Debug)]
pub struct FileAction {
    act: ActionDetermine,
    path: Box<Path>,
}

pub fn listner(
    action_sender: Sender<FileAction>,
    path: &str,
    config: Arc<SourceConfig>,
) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);

                //~ 로 시작하거나 끝나는 file은 임시파일로 간주, todo: 다른 예외사항도 있는지 알아볼 것.
                let is_temp_file = event
                    .paths
                    .last()
                    .and_then(|path| path.file_name())
                    .map(|name| {
                        name.as_encoded_bytes().starts_with(b"~")
                            || name.as_encoded_bytes().ends_with(b"~")
                    })
                    .expect("No file name found.");

                if !(!config.reflect_temporary_file && is_temp_file) {
                    //임시파일 적용 여부 확인
                    match event.kind {
                        EventKind::Create(_) => {
                            let mut ad = ActionDetermine::Unknown;
                            let event_path: Box<Path> =
                                Box::from(event.paths.last().expect("no path").as_path());
                            if event_path.is_dir() {
                                ad = ActionDetermine::CreateDir;
                            } else {
                                ad = ActionDetermine::CreateFile;
                            }
                            action_sender
                                .send(FileAction {
                                    act: ad,
                                    path: event_path,
                                })
                                .unwrap()
                        }
                        EventKind::Modify(_) => {
                            //modify 종류는 Modify(Name(From)), Modify(Name(To)),Modify(Any) 형식으로 받는다.
                            //여기서는 인식만 하고 Controller에서 From To 를 인식하여 바꾸는 방식으로.
                        }
                        EventKind::Remove(_) => {
                            //인식할 지 말지는 config 로부터 받아온다.
                        }
                        _ => {} // EventKind::Access(_) => {}
                                // EventKind::Any => {}
                                // EventKind::Other => {}
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
