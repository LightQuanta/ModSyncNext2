use std::{
    collections::HashMap,
    error::Error,
    fs::{self, DirEntry, File},
    io::BufReader,
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::Mutex,
    time::UNIX_EPOCH,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    let current_dir = minecraft_path();
    let current_dir = current_dir.to_str().unwrap_or(&current_relative_dir);
    if path.starts_with(&current_dir) {
        let mut path = path.to_str().unwrap_or(&current_relative_dir).to_string();
        path.replace_range(..(current_dir.len() + 1), &current_relative_dir);
        return Some(path);
    }
    Some(path.to_str()?.to_string())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileHashInfo {
    name: String,
    size: u64,
    #[serde(rename = "lastModified")]
    last_modified: u128,
    hash: String,
}

fn read_hash_cache() -> Option<HashMap<String, FileHashInfo>> {
    let json = fs::read_to_string(minecraft_path().join("MSN/hash.json"))
        .unwrap_or("cache file not found!".to_string());
    let parsed = serde_json::from_str(&json);
    match parsed {
        Ok(map) => {
            eprintln!("HASH = {:?}", map);
            Some(map)
        }
        Err(e) => {
            eprintln!("error while reading hash cache: {:?}", e);
            None
        }
    }
}

lazy_static! {
    static ref HASH: Mutex<HashMap<String, FileHashInfo>> =
        Mutex::new(read_hash_cache().unwrap_or(HashMap::new()));
}

fn get_mod_info(f: DirEntry) -> Result<Option<FileHashInfo>, Box<dyn Error>> {
    // 必须是文件
    if !f.file_type().map_or(false, |f| f.is_file()) {
        return Ok(None);
    }

    // 获取元数据
    let metadata = f.metadata()?;

    // 文件大小（字节）
    let size = metadata.file_size();

    // 修改日期（毫秒）
    let last_modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_millis();

    let path = to_relative_path_string(&f.path());
    if path.is_none() {
        return Ok(None);
    }
    let path = path.unwrap();
    eprintln!("relative_path = {:?}", path);

    // TODO 缓存path
    let mut hashmap = HASH.lock().unwrap();

    // 检查缓存
    if hashmap.contains_key(&path) {
        let fhi = hashmap.get(&path).unwrap().clone();
        return Ok(Some(fhi));
    } else {
        // 大写sha256
        let hash = compute_sha256(Path::new(&f.path()).to_path_buf())?;
        let s = FileHashInfo {
            name: f.file_name().to_string_lossy().to_string(),
            size,
            last_modified,
            hash,
        };

        hashmap.insert(
            path,
            s.clone(),
        );
        // TODO 保存hashmap缓存

        Ok(Some(s))
    }
}

fn compute_sha256(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    hasher.update(reader.buffer());

    Ok(format!("{:X}", hasher.finalize()))
}

pub fn get_mods_info(version: String) -> Vec<FileHashInfo> {
    let mod_folder = minecraft_path()
        .join(".minecraft")
        .join("versions")
        .join(version)
        .join("mods");

    eprintln!("mod_folder = {:?}", mod_folder);
    let mods = fs::read_dir(mod_folder);

    if let Ok(mods) = mods {
        return mods
            .filter_map(|f| {
                if !f.is_ok() {
                    eprintln!("F NOT OK");
                    return None;
                }
                let f = f.unwrap();
                let mod_info = get_mod_info(f);

                if let Ok(Some(mod_info)) = mod_info {
                    return Some(mod_info);
                }
                return None;
            })
            .collect();
    }
    eprintln!("NOT OK? {:?}", mods.unwrap_err());
    vec![]
}

pub fn choose_file() -> Option<String> {
    // 选择文件
    let file = rfd::FileDialog::new()
        .set_directory(minecraft_path())
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
                    if f.file_type().map_or(false, |f| f.is_dir()) {
                        return Some(f.path().file_name()?.to_str()?.to_string());
                    }
                    return None;
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
