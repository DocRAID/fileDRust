use crate::configure::{Config, SourceConfig};
use log::info;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};
use std::thread;

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

pub fn remote_sync(config: Config) {
    let (action_sender, action_receiver) = mpsc::channel(); //이거 네이밍을 다르게 해보자.
    info!("fileDRust remote agent started.");

    let mut handles = Vec::new();

    let source = Arc::new(config.source.clone());
    //source 측 감지 할 경로 리스트
    let listen_path_list = config.source.path_list.clone().unwrap();

    // path list 만큼 스레드 생성 후 변경 리스너 실행
    for dir in listen_path_list {
        let action_sender = action_sender.clone();
        let dir = dir.clone().to_string();
        let source = Arc::clone(&source);
        handles.push(thread::spawn(move || {
            listner(action_sender, &*dir, source).unwrap();
        }));
    }

    loop {
        if let Ok(x) = action_receiver.recv() {
            println!("@testlog:{:?}", x);
        }
    }
    // ctrlc::set_handler(move || {
    //     //스레드 헨들 회수
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    // }).expect("Error setting Ctrl-C handler");
}
fn listner(action_sender: Sender<FileAction>, path: &str, config: Arc<SourceConfig>) -> Result<()> {
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

fn controller() {
    //     todo: 하나의 프로세서로 분리.
    //     todo: 외부 서버 연결
    //     todo: create 적용
    //     todo: modify 업데이트
    //     todo: modify name 업데이트
    //     todo: 설정에 따라 delete 리스트 queue에서 관리하여 삭제
}
