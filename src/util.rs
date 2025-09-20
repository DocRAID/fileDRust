use crate::configure::Config;
use log::info;
use sha2::{Digest, Sha256};
use ssh2::Session;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fs, io, thread};

pub fn calculate_hash(path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

//ssh 관련
pub struct RemoteSync {
    session: Option<Session>,
    config: Config,
}
impl RemoteSync {
    pub fn new(config: Config) -> Self {
        Self {
            session: None,
            config,
        }
    }
    pub fn connect(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let target_config = &self.config.target;

        let ip = target_config
            .target_ip
            .as_ref()
            .ok_or("Target IP not configured")?;
        let user = target_config
            .target_user
            .as_ref()
            .ok_or("Target user not configured")?;
        let password = target_config
            .target_password
            .as_ref()
            .ok_or("Target password not configured")?;
        let working_dir = target_config
            .target_working_dir
            .as_ref()
            .ok_or("Target working directory not configured")?;

        let tcp = TcpStream::connect(format!("{}:22", ip))?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(user, password)?;

        self.session = Some(session);
        info!("Successfully connected to remote server: {}@{}", user, ip);
        Ok(())
    }
    // pub fn validate_path_access(self, target_working_dir: Box<Path>) -> bool {
    //     // 원격 디렉토리 존재 확인 및 생성
    //     let sftp = self.session.expect("invalid_session").sftp();
    //
    // }
}


