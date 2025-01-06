mod configure;
mod controler;
mod listener;
mod logging;

use log::info;
use notify::{Result, Watcher};
use crate::configure::{get_config};
use crate::controler::controler;
use crate::listener::listner;
use crate::logging::log_init;

fn main() -> Result<()> {
    let config = match get_config() {
        Ok(conf) => {conf}
        Err(_) => {
            panic!("config error.");
        }
    };
    let _log_handle = log_init("log/fileDRust.log".parse().unwrap());
    info!("fileDRust agent started.");
    // println!("{:?}",config);

    // export to process
    let _ = listner("./test");
    controler();

    // monitor process?

    Ok(())
}