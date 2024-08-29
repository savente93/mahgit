use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        /// the directory where to initialize the repository
        dir: Option<PathBuf>,
    },
    CatFile {
        #[arg(short, long)]
        /// pretty print the object based on it's type
        pretty_print: bool,

        /// the SHA-1 hash of the object we wish to read
        object: String,
    },
    HashObject {
        path: PathBuf,

        #[arg(short, long)]
        write: bool,
    },
}
