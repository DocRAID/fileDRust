mod configure;
mod controler;
mod listener;
mod logging;

use crate::configure::get_config;
use crate::controler::controler;
use crate::listener::{listner, FileAction};
use crate::logging::log_init;
use log::info;
use notify::{Result, Watcher};
use std::path::Path;
use std::sync::mpsc::RecvError;
use std::sync::{mpsc, Arc};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let (action_sender, action_receiver) = mpsc::channel(); //이거 네이밍을 다르게 해보자.

    let config: configure::Config = match get_config() {
        Ok(conf) => conf,
        Err(_) => {
            panic!("config error.");
        }
    };

    let _log_handle = log_init(r"log/fileDRust.log".parse().unwrap());
    info!("fileDRust agent started.");

    // export to process
    //스레드 핸들 리스트
    let mut handles = Vec::new();
    //소스 측의 config 불러오기
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

    // controler();

    //test loop
    loop {
        if let Ok(x) = action_receiver.recv() {
            println!("@testlog:{:?}", x);
        }
    }

    ctrlc::set_handler(move || {
        for handle in handles {
            handle.join().unwrap();
        }
    })
    .expect("Error setting Ctrl-C handler");

    Ok(())
}
