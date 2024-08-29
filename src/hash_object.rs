use anyhow::Result;
use std::{fs, path::PathBuf};

use sha1::{Digest, Sha1};

pub fn hash_object(path: PathBuf, _write: bool) -> Result<()> {
    let file_bytes = fs::read(path)?;
    let sha = Sha1::digest(file_bytes);

    println!("file content sha: {:?}", &sha[..]);
    Ok(())
}
