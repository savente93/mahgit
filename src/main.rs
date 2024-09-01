mod cat_file;
mod cli;
mod hash_object;
mod init;
mod io;
mod utils;

use std::fs;

use anyhow::Result;
use cat_file::cat_file;
use cli::*;
use hash_object::{sha1_blob, write_object_to_db};
use init::init_repo;

use clap::Parser;
fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { dir } => init_repo(dir),
        Commands::CatFile {
            pretty_print,
            object,
        } => {
            let contents = cat_file(object)?;
            let _ = pretty_print;
            print!("{}", contents);
            Ok(())
        }
        Commands::HashObject { path, write } => {
            let file_bytes = fs::read(path)?;
            let hash = sha1_blob(&file_bytes)?;
            print!("{:x?}", hash);
            if write {
                write_object_to_db(hash, file_bytes)
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {}
