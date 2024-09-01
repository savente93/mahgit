use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

use std::io::prelude::*;

pub fn read_zlib_file(path: PathBuf) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(&file);
    let mut decoded_file_contents = String::new();
    decoder.read_to_string(&mut decoded_file_contents)?;
    Ok(decoded_file_contents)
}
