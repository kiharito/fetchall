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
    Rm { index: u32 },
    /// Show the list of the target directory paths
    Ls {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { path }) => {
            println!("Added: {}", path);
        }
        Some(Commands::Rm { index }) => {
            println!("Removed: {}", index);
        }
        Some(Commands::Ls {}) => {
            println!("ls!");
        }
        None => {
            println!("fetch all!!")
        }
    }
}
