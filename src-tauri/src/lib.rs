use std::{env, fs};

use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    version: String,
    sync: Sync,
    minecraft: Minecraft,
}

impl Config {
    pub fn default() -> Config {
        Config {
            version: "2.0".to_string(),
            sync: Sync {
                server: "".to_string(),
                auto_update: false,
                auto_sync: false,
                action_after_sync: ActionAfterSync::DoNothing,
                command: "".to_string(),
            },
            minecraft: Minecraft {
                version: "".to_string(),
                isolate: false,
                sync_config: false,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum ActionAfterSync {
    Exit,
    DoNothing,
    ExecuteCommand,
    ExecuteCommandAndExit,
}

#[derive(Deserialize, Serialize)]
pub struct Sync {
    server: String,
    #[serde(rename = "autoUpdate")]
    auto_update: bool,
    #[serde(rename = "autoSync")]
    auto_sync: bool,
    #[serde(rename = "actionAfterSync")]
    action_after_sync: ActionAfterSync,
    command: String,
}

#[derive(Deserialize, Serialize)]
pub struct Minecraft {
    version: String,
    isolate: bool,
    #[serde(rename = "syncConfig")]
    sync_config: bool,
}

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "./msnconfig.txt";

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "./sample_config.toml";

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_exists = fs::metadata(CONFIG_PATH).is_ok();

    let config_text = if !config_exists {
        let default_config = Config::default();
        fs::write(CONFIG_PATH, toml::to_string(&default_config)?)?;
        toml::to_string(&default_config)?
    } else {
        fs::read_to_string(CONFIG_PATH)?
    };

    Ok(toml::from_str(&config_text)?)
}

pub fn select_file() -> Option<String> {
    let file = FileDialog::new()
        .add_filter("*", &["*.*"])
        .set_directory(env::current_dir().unwrap())
        .pick_file();
    if let Some(file) = file {
        return Some(file.to_str()?.to_string());
    }
    None
}
