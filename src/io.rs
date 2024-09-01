use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

use std::io::prelude::*;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::create_dir_all;

use crate::utils::path_from_object_name;

pub fn read_zlib_file(path: PathBuf) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(&file);
    let mut decoded_file_contents = String::new();
    decoder.read_to_string(&mut decoded_file_contents)?;
    Ok(decoded_file_contents)
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
