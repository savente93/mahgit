use crate::utils::path_from_object_name;
use anyhow::{anyhow, Result};

use crate::io::read_zlib_file;

pub fn cat_file(object: &str) -> Result<String> {
    let path = path_from_object_name(&object);

    if !path.exists() {
        return Err(anyhow!("Could not read file because it does not exist"));
    };

    let decoded_file_contents = read_zlib_file(path)?;
    let parts: Vec<&str> = decoded_file_contents.split("\x00").collect();
    let _kind_and_size = parts[0];
    let content = parts[1];
    Ok(content.to_string())
}
