use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Directory {
    path: String,
}

pub fn add(file_path: &str, path: String) -> Result<()> {
    let mut dirs = load_dirs(file_path)?;
    dirs.push(Directory { path });
    save_dirs(file_path, &dirs)
}

pub fn remove(file_path: &str, idx: usize) -> Result<()> {
    let mut dirs = load_dirs(file_path)?;
    if idx >= dirs.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Index"));
    }
    dirs.remove(idx);
    save_dirs(file_path, &dirs)
}

pub fn list(file_path: &str) -> Result<()> {
    let dirs = load_dirs(file_path)?;
    let mut idx = 0;
    for dir in dirs {
        println!("{}: {}", idx, dir.path);
        idx += 1;
    }
    Ok(())
}

pub fn fetchall(file_path: &str) -> Result<()> {
    let dirs = load_dirs(file_path)?;
    for dir in dirs {
        let path = dir.path;
        match Command::new("git").arg("fetch").current_dir(&path).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Fetch succeeded at {}", path);
                } else {
                    println!(
                        "Fetch failed at {}:\n{}",
                        path,
                        String::from_utf8(output.stderr).unwrap()
                    );
                }
            }
            Err(e) => {
                println!("Fetch failed at {}:\n{:?}", path, e);
            }
        };
    }
    Ok(())
}

fn load_dirs(file_path: &str) -> Result<Vec<Directory>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    match serde_json::from_reader(file) {
        Ok(dirs) => Ok(dirs),
        Err(e) if e.is_eof() => Ok(Vec::new()),
        Err(e) => Err(e)?,
    }
}

fn save_dirs(file_path: &str, dirs: &Vec<Directory>) -> Result<()> {
    let file = OpenOptions::new().write(true).open(file_path)?;
    file.set_len(0)?;
    serde_json::to_writer(file, &dirs)?;
    Ok(())
}
