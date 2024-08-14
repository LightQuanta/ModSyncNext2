use std::{
    collections::HashMap,
    error::Error,
    fs::{self, DirEntry, File},
    io::BufReader,
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
    time::UNIX_EPOCH,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub enum ActionAfterSync {
    Exit,
    DoNothing,
    ExecuteCommand,
    ExecuteCommandAndExit,
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
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

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

/// 替换当前配置
fn replace_current_config(config: Config) {
    let mut current_config = CONFIG.write().unwrap();
    *current_config = config;
}

pub fn has_config() -> bool {
    fs::metadata(CONFIG_PATH).is_ok()
}

pub fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let default_config = Config::default();
    replace_current_config(default_config.clone());
    fs::write(CONFIG_PATH, toml::to_string(&default_config).unwrap()).unwrap();
    Ok(())
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(CONFIG_PATH)?;
    let config: Config = toml::from_str(&config_text)?;
    replace_current_config(config.clone());
    Ok(config)
}

pub fn save_config(config: String) -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(&config)?;
    replace_current_config(config.clone());
    let toml_string = toml::to_string(&config)?;
    Ok(fs::write(CONFIG_PATH, toml_string)?)
}

/// 尝试将绝对路径转换为相对Minecraft文件夹下的路径
fn to_relative_path_string(path: &PathBuf) -> Option<String> {
    let current_relative_dir = ".".to_string() + std::path::MAIN_SEPARATOR_STR;
    // 对于同目录下的文件，计算相对路径
    let current_dir = minecraft_path();
    let current_dir = current_dir.to_str().unwrap_or(&current_relative_dir);

    // 移除同目录前的额外路径，替换为相对路径
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

/// 确保配置文件夹存在
fn make_config_dir() {
    let config_folder = minecraft_path().join("MSN");
    if !config_folder.exists() {
        if let Err(e) = fs::create_dir(config_folder) {
            eprintln!("Error while creating config folder: {:?}", e);
        }
    }
}

/// 读取MSN/hash.json中缓存的mod文件信息缓存
fn read_hash_cache() -> Option<HashMap<String, FileHashInfo>> {
    make_config_dir();
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

/// 将缓存的文件信息保存至MSN/hash.json
fn save_hash_cache() -> Result<(), Box<dyn Error>> {
    let hashmap = HASH.lock().unwrap().clone();
    let json = serde_json::to_string_pretty(&hashmap)?;
    Ok(fs::write(
        minecraft_path().join("MSN").join("hash.json"),
        json.as_bytes(),
    )?)
}

lazy_static! {
    static ref HASH: Mutex<HashMap<String, FileHashInfo>> =
        Mutex::new(read_hash_cache().unwrap_or(HashMap::new()));
}

/// 根据修改日期和文件大小检测mod是否发生变化
fn has_file_changed(file1: &FileHashInfo, file2: &FileHashInfo) -> bool {
    file1.last_modified != file2.last_modified || file1.size != file2.size
}

/// 获得某个mod的文件信息
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

    let mut hashmap = HASH.lock().unwrap();

    let mut current_file_info = FileHashInfo {
        name: f.file_name().to_string_lossy().to_string(),
        size,
        last_modified,
        hash: "temp".to_string(),
    };

    let cached_file_info = hashmap.get(&path);

    // 检查缓存
    if cached_file_info.is_some()
        && !has_file_changed(&cached_file_info.unwrap(), &current_file_info)
    {
        eprintln!("cached file {}", current_file_info.name);
        let fhi = cached_file_info.unwrap().clone();
        return Ok(Some(fhi));
    } else {
        eprintln!("new file {}", current_file_info.name);
        // 大写sha256
        let hash = compute_sha256(Path::new(&f.path()).to_path_buf())?;
        current_file_info.hash = hash;

        hashmap.insert(path, current_file_info.clone());

        Ok(Some(current_file_info))
    }
}

/// 计算并返回指定文件的大写SHA256字符串
fn compute_sha256(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    hasher.update(reader.buffer());

    Ok(format!("{:X}", hasher.finalize()))
}

/// 获得mods文件夹路径（支持版本隔离）
fn mods_path(version: String) -> PathBuf {
    if CONFIG.read().unwrap().minecraft.isolate {
        minecraft_path().join(".minecraft").join("mods")
    } else {
        minecraft_path()
            .join(".minecraft")
            .join("versions")
            .join(version)
            .join("mods")
    }
}

/// 获得某个版本的全部mod的文件信息
pub fn get_mods_info(version: String) -> Vec<FileHashInfo> {
    let mod_folder = mods_path(version);

    eprintln!("mod_folder = {:?}", mod_folder);
    let mods = fs::read_dir(mod_folder);

    if let Ok(mods) = mods {
        let mods_info = mods
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
        if let Err(e) = save_hash_cache() {
            eprintln!("Error saving hash cache: {:?}", e);
        }

        return mods_info;
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

/// 获得.minecraft/versions下的所有Minecraft版本名称
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

/// 获得.minecraft文件夹所在目录，优先读取环境变量MINECRAFT_PATH指定的目录，默认为当前目录
fn minecraft_path() -> PathBuf {
    let env = std::env::var("MINECRAFT_PATH");

    let path = if env.is_ok() {
        env.unwrap()
    } else {
        ".".to_string() + std::path::MAIN_SEPARATOR_STR
    };
    Path::new(&path).to_path_buf()
}
