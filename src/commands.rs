use crate::logic::{App, Space, get_config, get_config_path, save_config};
use clap::{Parser, Subcommand};
use rfd::FileDialog;
use std::{env::consts::OS, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about)]
pub struct InitArgs {
    /// Name for the new space
    space: String,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct AddArgs {
    /// Space to add this app to
    space: String,
    /// Label to recognize app
    label: String,
    /// Path to app; if provided, skips the file picker dialog
    path: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct ListArgs {
    /// If provided, lists apps in this space only
    space: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct OpenArgs {
    /// The name of the space whose apps you want to launch
    space: String,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct ConfigArgs {
    /// If provided, print config for this space only; `open` for file
    space: Option<String>,
}

/// Launch multiple apps from the CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new space
    Init(InitArgs),
    /// Launch all apps in a space
    Open(OpenArgs),
    /// Add a new app to a space
    Add(AddArgs),
    #[command(visible_alias = "ls")]
    /// List spaces and apps
    List(ListArgs),
    /// Show or open the config file
    Config(ConfigArgs),
}

fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory(match OS {
            "windows" => PathBuf::from("C:\\"),
            "macos" => PathBuf::from("/Applications"),
            "linux" => dirs::executable_dir().unwrap(),
            _ => dirs::home_dir().unwrap(),
        })
        .pick_file()
}

fn print_apps(space: &Space) {
    if !space.apps.is_empty() {
        print!("{}: ", space.name);
        for app in &space.apps {
            print!("'{}' ", app.label);
        }
        println!("");
    } else {
        println!("{} (No apps added)", space.name)
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
                apps: vec![],
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

            let exec_path = if args.path.is_some() {
                args.path.unwrap()
            } else {
                match pick_file() {
                    Some(path) => path.display().to_string(),
                    None => {
                        eprintln!("No executable selected.");
                        std::process::exit(1);
                    }
                }
            };

            space.apps.push(App {
                label: args.label,
                exec: exec_path,
            });

            save_config(c);

            println!("Added.");
        }
        Commands::Open(args) => {
            println!("Opening '{}'...", args.space);
            let c = get_config();

            let space = c
                .spaces
                .iter()
                .find(|s| s.name == args.space)
                .unwrap_or_else(|| {
                    eprintln!("Could not find a space named '{}'.", args.space);
                    std::process::exit(1);
                });

            if space.apps.is_empty() {
                eprintln!("No apps were added to '{}'.", space.name);
                std::process::exit(1);
            }

            print!("Apps: ");

            for app in &space.apps {
                match open::that(&app.exec) {
                    Ok(_) => print!("'{}' ", app.label),
                    Err(error) => {
                        eprint!("An error occurred: {:?}", error);
                        std::process::exit(1);
                    }
                }
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
        Commands::Config(args) => {
            let config = get_config();
            let config_path = get_config_path();

            if args.space.is_some() {
                let query = args.space.unwrap();

                if query == "open" {
                    open::that(config_path).unwrap_or_else(|error| {
                        eprint!("An error occurred: {:?}", error);
                        std::process::exit(1);
                    });

                    std::process::exit(0);
                }

                let space = config
                    .spaces
                    .iter()
                    .find(|s| s.name == query)
                    .unwrap_or_else(|| {
                        eprintln!("Could not find a space named '{}'.", query);
                        std::process::exit(1);
                    });

                println!(
                    "{}",
                    toml::to_string_pretty(space)
                        .expect(format!("Failed to deserialize '{}'.", query).as_str())
                );
            } else {
                println!(
                    "{}",
                    toml::to_string_pretty(&config).expect("Failed to deserialize configuration.")
                );
            }
        }
    }
    Ok(())
}
