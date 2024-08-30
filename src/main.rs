mod cat_file;
mod cli;
mod hash_object;
mod init;

use std::{fs::create_dir_all, io::Write};

use anyhow::Result;
use cat_file::{cat_file, path_from_object_name};
use cli::*;
use hash_object::{hash_object_on_disk, write_object_to_db};
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
            let hash = hash_object_on_disk(path)?;
            if write {
                write_object_to_db(hash, contents)
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {}
