mod configure;
mod controler;
mod listener;
mod logging;
mod ipc_module;

use std::sync::Arc;
use std::thread;
use log::info;
use notify::{Result, Watcher};
use crate::configure::{get_config};
use crate::controler::controler;
use crate::listener::listner;
use crate::logging::log_init;

fn main() -> Result<()> {
    let config:configure::Config = match get_config() {
        Ok(conf) => {conf}
        Err(_) => {
            panic!("config error.");
        }
    };
    let _log_handle = log_init(r"log/fileDRust.log".parse().unwrap());
    info!("fileDRust agent started.");

    // export to process
    let mut handles = Vec::new();
    let source = Arc::new(config.source.clone());
    let listen_list = config.source.path_list.clone().unwrap();
    for dir in listen_list {
        let dir = dir.clone().to_string();
        let source = Arc::clone(&source);
        handles.push(thread::spawn(move || {
            listner(&*dir, source).unwrap();
        }));
    }

    controler();

    for handle in handles {
        handle.join().unwrap();
    }

    // monitor process?

    Ok(())
}