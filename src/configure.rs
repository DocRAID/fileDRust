use std::{env, fs};
use serde::Deserialize;
use std::option::Option;

use toml::de::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub source: SourceConfig,
    pub target: TargetConfig,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub working_dir: Option<String>,
    pub allow_delete: bool,
    pub pending_delete_time: Option<String>, // min? hour?
    pub path_list: Option<Vec<String>>
}
#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub target_ip: Option<String>,
    pub user_password: Option<String>,
    pub target_password: Option<String>,
    pub target_working_dir: Option<String>,
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let mut path = env::current_dir().unwrap();
    path.push("config.toml");
    let config_contents = fs::read_to_string(path).expect("config.toml is not exist.");
    return toml::from_str(&config_contents)
}
