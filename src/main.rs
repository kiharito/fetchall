mod directories;
mod repository;

use crate::repository::JsonFileRepository;
use anyhow::Result;
use clap::{Parser, Subcommand};

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
    Rm { idx: usize },
    /// Show the list of the target directory paths
    Ls {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo = JsonFileRepository::new(".fetchall_dirs.json");

    match cli.command {
        Some(Commands::Add { path }) => directories::add(&repo, path)?,
        Some(Commands::Rm { idx }) => directories::remove(&repo, idx)?,
        Some(Commands::Ls {}) => directories::list(&repo)?,
        None => directories::fetchall(&repo)?,
    };

    Ok(())
}
