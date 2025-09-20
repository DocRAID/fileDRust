use crate::configure::{Config, SourceConfig};
use log::{info, log, trace, warn};
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use ssh2::Session;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

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

    let thread_pool = ThreadPool::new(config.system.applier_thread.unwrap()+1);
    // path list 만큼 스레드 생성 후 변경 리스너 실행
    for dir in listen_path_list {
        let action_sender = action_sender.clone();
        let dir = dir.clone().to_string();
        let source = Arc::clone(&source);
        handles.push(thread::spawn(move || {
            listner(action_sender, &dir, source).unwrap();
        }));
    }

    loop {
        match action_receiver.recv() {
            Ok(file_action) => {

                trace!("[SEND] New thread job : [{:?}] at {:?}", file_action.act, file_action.path);

                //활성 thread 수가 applier_thread를 넘지 않도록.
                while thread_pool.active_count() > config.system.applier_thread.unwrap() {
                    trace!("[SEND] Thread is saturated. Wait for the job to be finished");
                    thread::sleep(Duration::from_millis(1000));
                }

                thread_pool.execute({
                    // let task_name = file_action.clone();
                    move || {
                        trace!("[SEND] Running job thread for : [{:?}] at {:?}", file_action.act, file_action.path);
                        // 실제 작업 시뮬레이션 todo: 파일 전송
                        thread::sleep(Duration::from_secs(10));
                        trace!("[SEND] Finish job thread for : [{:?}] at {:?}", file_action.act, file_action.path);
                    }
                });
            }
            Err(_) => {
                warn!("[SEND] Warning thread input. must be shutdown");
                break;
            }
        }
        sleep(Duration::from_millis(10));
    }


    // Ctrl+C 핸들러 설정
    ctrlc::set_handler(move || {
        info!("Shutting down remote sync...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn listner(action_sender: Sender<FileAction>, path: &str, config: Arc<SourceConfig>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                trace!("event: {:?}", event);

                //~ 로 시작하거나 끝나는 file은 임시파일로 간주, todo: 다른 예외사항도 있는지 알아볼 것.
                let is_temp_file = event
                    .paths
                    .last()
                    .and_then(|path| path.file_name())
                    .map(|name| {
                        let name_bytes = name.as_encoded_bytes();
                        name_bytes.starts_with(b"~")
                            || name_bytes.ends_with(b"~")
                            || name_bytes.ends_with(b".tmp")
                            || name_bytes.ends_with(b".swp")
                            || name_bytes.ends_with(b".bak")
                    })
                    .unwrap_or(false);

                if config.reflect_temporary_file || !is_temp_file {
                    //임시파일 적용 여부 확인
                    match event.kind {
                        EventKind::Create(_) => {
                            let event_path: Box<Path> =
                                Box::from(event.paths.last().expect("no path").as_path());
                            let ad = if event_path.is_dir() {
                                ActionDetermine::CreateDir
                            } else {
                                ActionDetermine::CreateFile
                            };
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
                            let event_path: Box<Path> =
                                Box::from(event.paths.last().expect("no path").as_path());
                            if event_path.is_file() {
                                action_sender
                                    .send(FileAction {
                                        act: ActionDetermine::ModifyFile,
                                        path: event_path,
                                    })
                                    .unwrap();
                            }
                        }
                        EventKind::Remove(_) => {
                            if config.reflect_delete {
                                let event_path: Box<Path> =
                                    Box::from(event.paths.last().expect("no path").as_path());
                                action_sender
                                    .send(FileAction {
                                        act: ActionDetermine::Remove,
                                        path: event_path,
                                    })
                                    .unwrap();
                            }
                        }
                        _ => {
                            trace!("Unknown event at listen process: {:?}", event);
                        }
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
