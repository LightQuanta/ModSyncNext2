use std::{
    fs,
    path::{Path, PathBuf},
};

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

fn to_relative_path_string(path: &PathBuf) -> Option<String> {
    let current_relative_dir = ".".to_string() + std::path::MAIN_SEPARATOR_STR;
    // 对于同目录下的文件，计算相对路径
    let current_dir = current_dir();
    let current_dir = current_dir.to_str().unwrap_or(&current_relative_dir);
    if path.starts_with(&current_dir) {
        let mut path = path.to_str().unwrap_or(&current_relative_dir).to_string();
        path.replace_range(..(current_dir.len() + 1), &current_relative_dir);
        return Some(path);
    }
    Some(path.to_str()?.to_string())
}

fn current_dir() -> PathBuf {
    // 计算当前目录的相对和绝对路径
    let current_relative_dir = ".".to_string() + std::path::MAIN_SEPARATOR_STR;
    if let Ok(dir) = std::env::current_dir() {
        dir
    } else {
        Path::new(&current_relative_dir).to_path_buf()
    }
}

pub fn choose_file() -> Option<String> {
    // 选择文件
    let file = rfd::FileDialog::new()
        .set_directory(current_dir())
        .pick_file();

    to_relative_path_string(&file?.as_path().to_path_buf())
}

pub fn get_minecraft_versions() -> Vec<String> {
    let path = minecraft_path();

    let versions = path.join(".minecraft").join("versions");
    let versions = fs::read_dir(versions);

    if let Ok(files) = versions {
        return files
            .filter_map(|f| {
                if let Ok(f) = f {
                    return Some(f.path().file_name()?.to_str()?.to_string());
                };
                None
            })
            .collect();
    }
    vec![]
}

fn minecraft_path() -> PathBuf {
    let env = std::env::var("MINECRAFT_PATH");

    let path = if env.is_ok() {
        env.unwrap()
    } else {
        ".".to_string() + std::path::MAIN_SEPARATOR_STR
    };
    Path::new(&path).to_path_buf()
}
