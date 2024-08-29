use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

use std::io::prelude::*;

pub fn cat_file(object: String, pretty_print: bool) -> Result<()> {
    let _ = pretty_print;
    let dir = &object[0..2];
    let file_name = &object[2..];
    let path = PathBuf::new()
        .join(".git")
        .join("objects")
        .join(dir)
        .join(file_name);
    dbg!(&path);
    // println!("reading file at: {}", path.display());
    if !path.exists() {
        return Err(anyhow!("Could not read file because it does not exist"));
    };

    // let encoded_file_contents = fs::read(path)?;
    let file = std::fs::File::open(path)?;

    let mut decoder = ZlibDecoder::new(&file);
    let mut decoded_file_contents = String::new();
    decoder.read_to_string(&mut decoded_file_contents)?;

    let parts: Vec<&str> = decoded_file_contents.split("\x00").collect();
    let _kind_and_size = parts[0];
    let content = parts[1];
    print!("{}", content);
    Ok(())
}
