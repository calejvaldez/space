mod commands;
mod logic;
use crate::commands::no_subcommand;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();
    let cmd = cli.cmd;

    match cmd {
        None => {
            no_subcommand();
        }
        Some(cmd) => {
            if let Err(e) = commands::run(cmd) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
