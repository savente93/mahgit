use anyhow::Result;

use sha1::{Digest, Sha1};

pub fn sha1_blob(contents: &[u8]) -> Result<Vec<u8>> {
    let header = format!("blob {}\x00", contents.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(contents);
    let sha = hasher.finalize();
    Ok(sha.iter().cloned().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs::File;
    use std::io::{self, Write};
    use std::process::{Command, Stdio};
    use uuid::Uuid;

    // fn test_against_git_binary(contents: String) -> Result<()> {

    //     let tmp = std::env::temp_dir();
    //     let path = tmp.join(Uuid::new_v4().to_string());
    //     let mut file = File::open(path)?;
    //     file.write(contents.as_bytes());
    //     let _ = file;
    //     Ok(())
    // }
    // #[test]
    // fn test_sha1_blob_matches_git_impl() -> Result<()> {
    //     let test_cases: Vec<String> = vec![
    //         "hello world",
    //         "asdf",
    //         "حمد علي",
    //         "由紀夫 三島",
    //         "אברהם לינקול",
    //     ]
    //     .into_iter()
    //     .map(String::from)
    //     .collect();

    //     for t in test_cases.iter() {
    //         let
    //         let mut child = Command::new("git")
    //             .args(["hash-object", "--stdin"])
    //             .stdout(Stdio::piped())
    //             .stdin(Stdio::piped())
    //             .spawn()
    //             .unwrap();
    //         let child_stdin = child.stdin.as_mut().unwrap();
    //         child_stdin.write_all(t.as_bytes())?;
    //         // Close stdin to finish and avoid indefinite blocking
    //         let _ = child_stdin;

    //         let output = child.wait_with_output()?;

    //         let hash = sha1_blob(t.as_bytes())?;
    //         assert_eq!(hash, output.stdout, "{}", t);
    //     }
    //     Ok(())
    // }
}
