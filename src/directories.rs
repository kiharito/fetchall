use crate::repository::Repository;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    path: String,
}

pub fn add(repo: &impl Repository, path: String) -> Result<()> {
    if !Path::new(&path).is_dir() {
        return Err(anyhow!("No such directory"));
    }

    let abs_path = if Path::new(&path).is_absolute() {
        path
    } else {
        fs::canonicalize(path)?.to_string_lossy().into_owned()
    };

    match Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .current_dir(&abs_path)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                return Err(anyhow!("Not a git repository"));
            }
        }
        Err(e) => return Err(e.into()),
    }

    let mut dirs = repo.collect()?;
    match dirs.iter().find(|&dir| dir.path == abs_path) {
        Some(_) => {
            println!("Already exists");
            return Ok(());
        }
        None => dirs.push(Directory { path: abs_path }),
    }

    repo.save(&dirs)
}

pub fn remove(repo: &impl Repository, idx: usize) -> Result<()> {
    let mut dirs = repo.collect()?;
    if idx >= dirs.len() {
        return Err(anyhow!("Invalid index"));
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
        let mut sp = Spinner::new(Spinners::Line, format!("Fetching at {}", path));
        match Command::new("git").arg("fetch").current_dir(&path).output() {
            Ok(output) => {
                if output.status.success() {
                    sp.stop_and_persist("✔", format!("Fetching at {} ... Done!", path));
                } else {
                    sp.stop_and_persist("×", format!("Fetching at {} ... Failed!", path));
                    println!("{}", String::from_utf8(output.stderr).unwrap());
                }
            }
            Err(e) => {
                sp.stop_and_persist("×", format!("Fetching at {} ... Failed!", path));
                println!("{:?}", e);
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
