use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::{fs, io};

pub fn calculate_hash(path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
