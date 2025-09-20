mod configure;
mod local_recovery;
mod logging;
mod remote_sync;
mod util;

use crate::configure::{get_config, Config};
use crate::local_recovery::{perform_backup, perform_restore};
use crate::logging::log_init;
use crate::remote_sync::{remote_sync, FileAction};
use chrono::{Datelike, Local, Timelike};
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use log::{info, log, trace, warn};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::option::Option;
use std::process::exit;
use std::sync::mpsc::{RecvError, Sender};
use std::sync::{mpsc, Arc};
use std::{fs, io, path::PathBuf, time::SystemTime};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "fileDRust2")]
#[command(about = "Multipurpose backup tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 백업을 수행합니다.      example: backup --source </temp/source> --targets </temp/backup> </temp/backup2..>
    Backup {
        /// 원본 디렉토리
        #[arg(short, long)]
        source: Option<PathBuf>,

        /// 대상 디렉토리들 (2개 이상 권장)
        #[arg(short, long, num_args=1..10)]
        targets: Option<Vec<PathBuf>>,
    },
    /// 복원을 수행합니다.      example: restore --backup-path </TEMP/backup> --restore-to </TEMP/source>
    Restore {
        #[arg(short, long)]
        backup_path: PathBuf,

        #[arg(short, long)]
        restore_to: PathBuf,
    },
    /// 원격지와 실시간 백업을 수행합니다.
    Sync,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let _log_handle = log_init(r"log/fileDRust.log".parse().unwrap());
    trace!("=============== [ Start at : {}-{}-{} {}:{}:{} ] ================",
        Local::now().year(), Local::now().month(), Local::now().day(),
        Local::now().hour(), Local::now().minute(), Local::now().second());

    info!("fileDRust started.");
    trace!("{:?}", &cli);
    let (action_sender, action_receiver) = mpsc::channel::<FileAction>();

    let config: Option<configure::Config> = match get_config() {
        Some(conf) => Some(conf),
        None => {
            trace!("config.toml 설정이 유효하지 않습니다.");
            None
        }
    };

    match cli.command {
        Commands::Backup { source, targets } => {
            //todo: 설정값 적용
            let source = match source {
                Some(source) => source,
                _ => {
                    warn!("source 에 값이 없습니다.");
                    exit(0);
                }
            };
            let targets = targets.unwrap_or_default();
            if targets.is_empty() {
                warn!("백업 대상 디렉토리가 지정되지 않았습니다.");
                exit(0);
            }
            if targets.len() == 1 {
                warn!("Warning: 대상 디렉토리는 2개 이상을 권고합니다.");
            }
            perform_backup(source, targets)?;
        }
        Commands::Restore {
            backup_path,
            restore_to,
        } => {
            perform_restore(backup_path, restore_to)?;
        }
        Commands::Sync => match config {
            None => {
                info!("config.toml 설정파일이 없습니다.");
                exit(0);
            }
            Some(x) => {
                remote_sync(x);
            }
        },
    }

    Ok(())
}
