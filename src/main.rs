mod cat_file;
mod cli;
mod hash_object;
mod init;
use anyhow::Result;
use cat_file::cat_file;
use cli::*;
use hash_object::hash_object;
use init::init_repo;

use clap::Parser;
fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { dir } => init_repo(dir),
        Commands::CatFile {
            pretty_print,
            object,
        } => cat_file(object, pretty_print),
        Commands::HashObject { path, write } => hash_object(path, write),
    }
}

#[cfg(test)]
mod test {}
