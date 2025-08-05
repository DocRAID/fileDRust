use log::trace;
use serde::Deserialize;
use std::option::Option;
use std::{env, fs};
use toml::de::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub source: SourceConfig,
    pub target: TargetConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SourceConfig {
    //Local environment to detect change points
    pub working_dir: Option<String>,         //Working directory
    pub reflect_temporary_file: bool, //Whether it reflects temporary files (as sample.txt~, ~$n sample.docx)
    pub reflect_delete: bool,         // Whether it reflects file deletion and path deletion
    pub pending_delete_time: Option<String>, // min? hour? todo more explain..
    pub path_list: Option<Vec<String>>, //(optional) Path to detect under the working directory. Default: <working_dir>/
}
#[derive(Debug, Deserialize, Clone)]
pub struct TargetConfig {
    //External to apply change points
    pub target_ip: Option<String>,          // target connection ip
    pub target_user: Option<String>,        // target connection user
    pub target_password: Option<String>,    //target connection password
    pub target_working_dir: Option<String>, // Working directory
}
#[derive(Debug, Deserialize, Clone)]
pub struct SystemConfig {
    pub applier_thread: Option<usize>,
}

pub(crate) fn get_config() -> Option<Config> {
    let mut path = env::current_dir().unwrap();
    path.push("config.toml");
    let config_contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            trace!("config.toml 파일이 존재하지 않습니다.");
            return None;
        }
    };
    Some(toml::from_str(&config_contents).expect("REASON"))
}
