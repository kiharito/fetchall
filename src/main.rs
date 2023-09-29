use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.name {
        Some(name) => println!("Name: {}", name),
        None => println!("No Name..."),
    }
}
