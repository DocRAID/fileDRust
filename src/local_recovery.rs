use crate::util::calculate_hash;
use chrono::Local;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};
use walkdir::WalkDir;

pub fn perform_backup(source: PathBuf, targets: Vec<PathBuf>) -> io::Result<()> {
    let mut hash_map = HashMap::new();

    for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(&source).unwrap();
            let hash = calculate_hash(&path.to_path_buf())?;
            hash_map.insert(rel_path.to_owned(), hash.clone());

            for target in &targets {
                let target_path = target.join(rel_path);
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(path, &target_path)?;
            }
        }
    }

    println!("백업 완료: {}", Local::now());
    for (file, hash) in &hash_map {
        println!("{:?} => {}", file, hash);
    }

    Ok(())
}

pub fn perform_restore(backup_path: PathBuf, restore_to: PathBuf) -> io::Result<()> {
    for entry in WalkDir::new(&backup_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(&backup_path).unwrap();
            let dest_path = restore_to.join(rel_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, dest_path)?;
        }
    }
    println!("복원 완료: {}", Local::now());
    Ok(())
}
