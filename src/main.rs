use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

mod directories;

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
    let file_path = ".fetchall_dirs.json";

    match cli.command {
        Some(Commands::Add { path }) => match directories::add_directory_path(file_path, path) {
            Err(e) => panic!("Add failed: {}", e),
            _ => {}
        },
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
