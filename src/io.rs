use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

use std::io::prelude::*;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::create_dir_all;

use crate::hash_object::sha1_blob;
use crate::utils::path_from_object_name;

/// reads and decodes a zlib compressed file at `path`
/// files is assumed to contain a zlib header
pub fn read_zlib_file(path: PathBuf) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(&file);
    let mut decoded_file_contents = String::new();
    decoder.read_to_string(&mut decoded_file_contents)?;
    Ok(decoded_file_contents)
}

/// compresses `contents` using the zlib algorithm using the default paramters
/// and then writes it to disk, assuming that all the directories along the path
/// are created beforehand
pub fn write_zlib_file(path: PathBuf, contents: Vec<u8>) -> Result<()> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&contents)?;
    let compressed_bytes = e.finish()?;
    let mut file = std::fs::File::create(path)?;
    file.write_all(&compressed_bytes)?;
    Ok(())
}

/// writes the given object contents to a zlib compressed file on disk in the appropriate git db
/// it uses the SHA1 hash to determine where in the db it should be written (see `path_from_object_fname`)
/// and also writes the file in a zlib compressed format to disk (see `write_zlib_file`)
/// this function will create directories as necessary in contrast to `write_zlib_file`
/// returns the hash of the file that was written
pub fn write_object_to_db(contents: Vec<u8>) -> Result<String> {
    let hash = sha1_blob(&contents)?;
    let hash_str = String::from_utf8(hash.clone())?;
    // TODO get rid of this clone
    let path = path_from_object_name(&hash_str);
    create_dir_all(path.parent().unwrap())?;
    write_zlib_file(path, contents)?;
    Ok(hash_str)
}
