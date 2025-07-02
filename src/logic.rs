use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub spaces: Vec<Space>,
}

#[derive(Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    pub commands: Vec<App>,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub label: String,
    pub exec: String,
}

fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("A config folder could not be found.");
    config_dir.join("com.calejvaldez.space").join("config.toml")
}

pub fn get_config() -> Config {
    let config_dir = dirs::config_dir()
        .expect("A config folder could not be found.")
        .join("com.calejvaldez.space");
    let config_path = config_dir.join("config.toml");

    if !fs::exists(&config_path).unwrap() {
        let default_config = Config { spaces: vec![] };

        fs::create_dir_all(config_dir).unwrap_or_else(|error| {
            panic!("Something went wrong. Error: {}", error);
        });

        let default_content = toml::to_string(&default_config).unwrap();
        fs::write(&config_path, default_content)
            .expect(format!("Creating {:?} failed.", &config_path).as_str());

        default_config
    } else {
        let content = fs::read_to_string(&config_path).unwrap();
        let c: Config = toml::from_str(content.as_str())
            .expect(format!("{:?} is not structured correctly.", &config_path).as_str());
        c
    }
}

pub fn save_config(c: Config) {
    let config_path = get_config_path();
    fs::write(
        &config_path,
        toml::to_string(&c).expect("Converting from configuration to string failed."),
    )
    .expect(format!("Saving {:?} failed.", &config_path).as_str());
}
