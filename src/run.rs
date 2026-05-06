use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "shard-run")]
#[command(about = "Shard Runtime")]
struct Cli {
    #[arg(help = "Input file")]
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    
    println!("Shard Runtime");
    println!("Running: {}", cli.input.display());
    
    let status = std::process::Command::new(&cli.input)
        .status()
        .expect("Failed to execute");
    
    std::process::exit(status.code().unwrap_or(1));
}
