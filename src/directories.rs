use crate::repository::Repository;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    path: String,
}

pub fn add(repo: &impl Repository, path: String) -> Result<()> {
    let mut dirs = repo.collect()?;
    dirs.push(Directory { path });
    repo.save(&dirs)
}

pub fn remove(repo: &impl Repository, idx: usize) -> Result<()> {
    let mut dirs = repo.collect()?;
    if idx >= dirs.len() {
        return Err(anyhow!("Invalid Index"));
    }
    dirs.remove(idx);
    repo.save(&dirs)
}

pub fn list(repo: &impl Repository) -> Result<()> {
    let dirs = repo.collect()?;
    let mut idx = 0;
    for dir in dirs {
        println!("{}: {}", idx, dir.path);
        idx += 1;
    }
    Ok(())
}

pub fn fetchall(repo: &impl Repository) -> Result<()> {
    let dirs = repo.collect()?;
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
