use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = false)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a directory path to the targets
    Add { path: String },
    /// Remove the specified directory path from the targets
    Rm { index: u32 },
    /// Show the list of the target directory paths
    Ls {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { path }) => {
            let mut file = match OpenOptions::new().write(true).create(true).append(true).open("fetchall_dirs.txt") {
                Err(e) => panic!("File open error: {}", e),
                Ok(file) => file,
            };
            let path_str = path + "\n";
            match file.write_all(path_str.as_ref()) {
                Err(e) => panic!("File write error: {}", e),
                Ok(_) => println!("successfully added"),
            };
        }
        Some(Commands::Rm { index }) => {
            println!("Removed: {}", index);
        }
        Some(Commands::Ls {}) => {
            // ファイルがないとき空ファイルを生成(write権限がないとcreateできない)
            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open("fetchall_dirs.txt")
            {
                Err(e) => panic!("File open error: {}", e),
                Ok(file) => file,
            };
            let mut str = String::new();
            match file.read_to_string(&mut str) {
                Err(e) => panic!("File read error: {}", e),
                Ok(_) => {
                    if str.is_empty() {
                        println!("-- empty --")
                    } else {
                        println!("{}", str)
                    }
                }
            }
        }
        None => {
            let result = Command::new("git").arg("fetch").output();
            match result {
                Ok(res) => {
                    if res.status.success() {
                        println!("Fetch success");
                    } else {
                        println!("Fetch failed");
                    }
                }
                _ => {
                    println!("Fetch failed");
                }
            }
        }
    }
}
