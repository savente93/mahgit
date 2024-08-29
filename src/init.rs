use anyhow::{anyhow, Result};
use std::{fs, path::PathBuf};

pub fn init_repo(dir: Option<PathBuf>) -> Result<()> {
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
