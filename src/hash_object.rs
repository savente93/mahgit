use anyhow::Result;
use std::{fs, path::PathBuf};

use sha1::{Digest, Sha1};

pub fn hash_object(path: PathBuf) -> Result<Vec<u8>> {
    let file_bytes = fs::read(path)?;
    let header = format!("blob {}\x00", file_bytes.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(file_bytes);
    let sha = hasher.finalize();
    Ok(sha.iter().cloned().collect())
}
