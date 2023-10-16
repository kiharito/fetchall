use anyhow::Result;
use clap::{Parser, Subcommand};

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
    Rm { idx: usize },
    /// Show the list of the target directory paths
    Ls {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file_path = ".fetchall_dirs.json";

    match cli.command {
        Some(Commands::Add { path }) => directories::add(file_path, path)?,
        Some(Commands::Rm { idx }) => directories::remove(file_path, idx)?,
        Some(Commands::Ls {}) => directories::list(file_path)?,
        None => directories::fetchall(file_path)?,
    };

    Ok(())
}
