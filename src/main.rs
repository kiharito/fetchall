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
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a directory to the targets
    Add { path: String },
    /// Remove the specified directory from the targets
    Rm { idx: usize },
    /// Show the list of the target directories
    Ls {},
    /// Run `git fetch` on all target directories
    Exec { options: Option<Vec<String>> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo = JsonFileRepository::new(".fetchall_dirs.json");

    match cli.command {
        Commands::Add { path } => directories::add(&repo, path)?,
        Commands::Rm { idx } => directories::remove(&repo, idx)?,
        Commands::Ls {} => directories::list(&repo)?,
        Commands::Exec { options } => directories::exec(&repo, options)?,
    };

    Ok(())
}
