use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::{fs, path::PathBuf};

use sha1::{Digest, Sha1};

use std::io::prelude::*;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { dir } => {
            let root = match dir {
                Some(p) => p,
                None => PathBuf::new(),
            };
            let db_dir = root.join(".git");
            if !&db_dir.exists() || fs::read_dir(&db_dir)?.next().is_none() {
                fs::create_dir(".git").unwrap();
                fs::create_dir(".git/objects").unwrap();
                fs::create_dir(".git/refs").unwrap();
                fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
                println!("Initialized git directory");
                Ok(())
            } else {
                Err(anyhow!(
                    "Directory {} exists and is non empty, aborting!",
                    db_dir.canonicalize()?.display()
                ))
            }
        }
        Commands::CatFile {
            pretty_print,
            object,
        } => {
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
        Commands::HashObject { path, write } => {
            let file_bytes = fs::read(path)?;
            let sha = Sha1::digest(file_bytes);

            println!("file content sha: {:?}", &sha[..]);
            Ok(())
        }
    }
}
