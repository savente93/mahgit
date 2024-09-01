use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::create_dir_all;

use sha1::{Digest, Sha1};
use std::io::prelude::*;

use crate::utils::path_from_object_name;

pub fn sha1_blob(contents: &[u8]) -> Result<Vec<u8>> {
    let header = format!("blob {}\x00", contents.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(contents);
    let sha = hasher.finalize();
    Ok(sha.iter().cloned().collect())
}

pub fn write_object_to_db(hash: Vec<u8>, contents: Vec<u8>) -> Result<()> {
    let hash_str = String::from_utf8(hash.clone())?;
    let path = path_from_object_name(hash_str);
    create_dir_all(path.parent().unwrap())?;
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&contents)?;
    let compressed_bytes = e.finish()?;
    let mut file = std::fs::File::create(path)?;
    file.write_all(&compressed_bytes)?;
    Ok(())
}
