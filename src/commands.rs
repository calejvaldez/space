use crate::logic::{App, Space, get_config, save_config};
use clap::{Parser, Subcommand};
use rfd::FileDialog;
use std::{env::consts::OS, path::PathBuf, process::Command};

#[derive(Parser)]
#[command(author, version, about)]
pub struct InitArgs {
    space: String,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct AddArgs {
    space: String,
    label: String,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct ListArgs {
    space: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct LaunchArgs {
    space: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Launch(LaunchArgs),
    Add(AddArgs),
    List(ListArgs),
}

fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory(match OS {
            "macos" => PathBuf::from("/Applications"),
            "linux" => dirs::executable_dir().unwrap(),
            _ => dirs::home_dir().unwrap(),
        })
        .pick_file()
}

fn run_app(exec: &String) {
    Command::new(match OS {
        "windows" => "start",
        "macos" => "open",
        "linux" => "xdg-open",
        _ => {
            eprintln!("This operating system is not supported.");
            std::process::exit(1);
        }
    })
    .args([&exec])
    .spawn()
    .unwrap_or_else(|error| {
        eprint!("An error occurred: {:?}", error);
        std::process::exit(1);
    });
}

fn print_apps(space: &Space) {
    if space.commands.len() > 0 {
        println!("Space: {}", space.name);
        for command in &space.commands {
            println!("- {} (`{}`)", command.label, command.exec);
        }
    } else {
        println!("Space: {} (No apps added)", space.name)
    }
}

pub fn run(cmd: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::Init(args) => {
            println!("Creating space '{}'...", &args.space);

            let mut c = get_config();

            if c.spaces.iter().any(|s| s.name == args.space) {
                eprintln!(
                    "'{}' already exists. Please try a different name.\n\nHint: `space list` will list all spaces.",
                    args.space
                );
                std::process::exit(1);
            }

            c.spaces.push(Space {
                name: args.space,
                commands: vec![],
            });
            save_config(c);

            println!("Done.");
        }
        Commands::Add(args) => {
            println!("Adding '{}' to {}...", args.label, args.space);

            let mut c = get_config();
            let space = c
                .spaces
                .iter_mut()
                .find(|s| s.name == args.space)
                .unwrap_or_else(|| {
                    eprintln!("Could not find a space named '{}'.", args.space);
                    std::process::exit(1);
                });

            space.commands.push(App {
                label: args.label,
                exec: match pick_file() {
                    Some(path) => path.display().to_string(),
                    None => {
                        eprintln!("No executable selected.");
                        std::process::exit(1);
                    }
                },
            });

            save_config(c);

            println!("Added.");
        }
        Commands::Launch(args) => {
            println!("Launching '{}'...", args.space);
            let c = get_config();

            let space = c
                .spaces
                .iter()
                .find(|s| s.name == args.space)
                .unwrap_or_else(|| {
                    eprintln!("Could not find a space named '{}'.", args.space);
                    std::process::exit(1);
                });

            if space.commands.len() == 0 {
                eprintln!("No apps were added to '{}'.", space.name);
                std::process::exit(1);
            }

            print!("Apps: ");

            for command in &space.commands {
                print!("{} ", command.label);
                run_app(&command.exec);
            }
            print!("\n");

            println!("Done.")
        }
        Commands::List(args) => {
            let c = get_config();

            if args.space.is_some() {
                let value = args.space.unwrap();
                let space = c
                    .spaces
                    .iter()
                    .find(|s| s.name == value)
                    .unwrap_or_else(|| {
                        eprintln!("Could not find a space named '{}'.", value);
                        std::process::exit(1);
                    });

                print_apps(space);
            } else {
                for space in &c.spaces {
                    print_apps(space);
                }
            }
        }
    }
    Ok(())
}
