mod commands;
mod logic;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = commands::run(cli.cmd) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
