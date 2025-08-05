use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use std::path::PathBuf;

pub fn log_init(path_buf: PathBuf) -> log4rs::Handle {
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] [{I}]: {m}{n}",
        )))
        .build(path_buf)
        .expect("Failed to create file appender");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}]:{m}{n}")))
        .build();

    let file_appender = Appender::builder().build("file", Box::new(file_appender));

    let console_appender = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
        .build("console", Box::new(stdout));

    let config = Config::builder()
        .appender(file_appender)
        .appender(console_appender)
        .build(
            Root::builder()
                .appender("file")
                .appender("console")
                .build(LevelFilter::Trace),
        )
        .expect("Failed to build log config");

    log4rs::init_config(config).expect("Log initialization failed")
}
