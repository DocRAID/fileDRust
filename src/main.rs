mod configure;
mod controler;
mod listener;

use notify::{Result, Watcher};
use crate::configure::{get_config};
use crate::controler::controler;
use crate::listener::listner;

fn main() -> Result<()> {
    let config = match get_config() {
        Ok(conf) => {conf}
        Err(_) => {panic!("config error.")}
    };
    println!("{:?}",config);


    // export to process
    let _ = listner("./test");
    controler();

    // monitor process?

    Ok(())
}