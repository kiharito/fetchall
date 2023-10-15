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

fn main() {
    let cli = Cli::parse();
    let file_path = ".fetchall_dirs.json";

    match cli.command {
        Some(Commands::Add { path }) => match directories::add(file_path, path) {
            Err(e) => panic!("Add failed: {}", e),
            _ => {}
        },
        Some(Commands::Rm { idx }) => match directories::remove(file_path, idx) {
            Err(e) => panic!("Remove failed: {}", e),
            _ => {}
        },
        Some(Commands::Ls {}) => match directories::list(file_path) {
            Err(e) => panic!("List failed: {}", e),
            _ => {}
        },
        None => match directories::fetchall(file_path) {
            Err(e) => panic!("Fetchall failed: {}", e),
            _ => {}
        },
    }
}
