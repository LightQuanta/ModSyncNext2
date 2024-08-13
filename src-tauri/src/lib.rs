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
const CONFIG_PATH: &str = "../sample_config.toml";

pub fn has_config() -> bool {
    fs::metadata(CONFIG_PATH).is_ok()
}

pub fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let default_config = Config::default();
    fs::write(CONFIG_PATH, toml::to_string(&default_config).unwrap()).unwrap();
    Ok(())
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(CONFIG_PATH)?;
    Ok(toml::from_str(&config_text)?)
}

pub fn save_config(config: String) -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(&config)?;
    let toml_string = toml::to_string(&config)?;
    Ok(fs::write(CONFIG_PATH, toml_string)?)
}

pub fn choose_file() -> Option<String> {
    // 计算当前目录的相对和绝对路径
    let current_relative_dir = &(".".to_string() + std::path::MAIN_SEPARATOR_STR);
    let current_dir = if let Ok(dir) = std::env::current_dir() {
        &dir.to_str()?.to_string()
    } else {
        current_relative_dir
    };

    // 选择文件
    let file = rfd::FileDialog::new()
        .set_directory(current_dir)
        .pick_file();
    let mut path = file?.as_path().to_str()?.to_string();

    // 对于同目录下的文件，计算相对路径
    if path.starts_with(current_dir) {
        path.replace_range(..(current_dir.len() + 1), current_relative_dir);
    }

    Some(path)
}
