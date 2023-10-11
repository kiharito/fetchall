use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result};

#[derive(Serialize, Deserialize, Debug)]
struct Directory {
    path: String,
}

pub fn add(file_path: &str, path: String) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    let mut dirs: Vec<Directory> = match serde_json::from_reader(file) {
        Ok(dirs) => dirs,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    dirs.push(Directory { path });
    let file = OpenOptions::new().write(true).open(file_path)?;
    serde_json::to_writer(file, &dirs)?;
    Ok(())
}

pub fn remove(file_path: &str, idx: usize) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    let mut dirs: Vec<Directory> = match serde_json::from_reader(file) {
        Ok(dirs) => dirs,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    if idx >= dirs.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Index"));
    }
    dirs.remove(idx);
    let file = OpenOptions::new().write(true).open(file_path)?;
    file.set_len(0)?;
    serde_json::to_writer(file, &dirs)?;
    Ok(())
}

pub fn list(file_path: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    let dirs: Vec<Directory> = match serde_json::from_reader(file) {
        Ok(dirs) => dirs,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    let mut idx = 0;
    for dir in dirs {
        println!("{}: {}", idx, dir.path);
        idx += 1;
    }
    Ok(())
}
