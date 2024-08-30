use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

use std::io::prelude::*;

pub fn cat_file(object: String) -> Result<String> {
    let path = path_from_object_name(object);

    if !path.exists() {
        return Err(anyhow!("Could not read file because it does not exist"));
    };

    let decoded_file_contents = read_zlib_file(path)?;
    let parts: Vec<&str> = decoded_file_contents.split("\x00").collect();
    let _kind_and_size = parts[0];
    let content = parts[1];
    Ok(content.to_string())
}

fn read_zlib_file(path: PathBuf) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(&file);
    let mut decoded_file_contents = String::new();
    decoder.read_to_string(&mut decoded_file_contents)?;
    Ok(decoded_file_contents)
}

pub fn path_from_object_name(object: String) -> PathBuf {
    let dir = &object[0..2];
    let file_name = &object[2..];
    let path = PathBuf::new()
        .join(".git")
        .join("objects")
        .join(dir)
        .join(file_name);

    path
}
