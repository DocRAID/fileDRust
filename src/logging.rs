use std::path::PathBuf;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn log_init(path_buf: PathBuf) -> log4rs::Handle {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] [{I}]: {m}{n}")))
        .build();

    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] [{i}]: {m}{n}")))
        .build(path_buf)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .build("logfile", Box::new(logfile))
        )
        .appender(
            Appender::builder()
                .build("stdout", Box::new(stdout))
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Info),
        )
        .unwrap();

    return log4rs::init_config(config).expect("log setting failed");
}