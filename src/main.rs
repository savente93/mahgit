mod cat_file;
mod cli;
mod hash_object;
mod init;
mod object;

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
        } => {
            let contents = cat_file(object)?;
            let _ = pretty_print;
            print!("{}", contents);
            Ok(())
        }
        Commands::HashObject { path, write } => {
            let hash = hash_object(path)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {}
