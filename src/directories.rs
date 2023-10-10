use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::hash::Hasher;
use std::io::{Read, Result, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Directory {
    path: String,
}

pub fn add_directory_path(file_path: &str, path: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    let mut dirs: Vec<Directory> = serde_json::from_reader(file)?;
    dirs.push(Directory { path });
    let dirs_str = serde_json::to_string(&dirs).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;
    serde_json::to_writer(file, &dirs_str)?;
    Ok(())
}
