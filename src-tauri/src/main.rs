// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use modsyncnext2::FileHashInfo;

#[tauri::command]
fn has_config() -> bool {
    modsyncnext2::has_config()
}

#[tauri::command]
fn create_default_config() -> bool {
    modsyncnext2::create_default_config().is_ok()
}

#[tauri::command]
fn choose_file() -> Option<String> {
    modsyncnext2::choose_file()
}

#[tauri::command]
fn read_config() -> String {
    match modsyncnext2::read_config() {
        Ok(config) => match serde_json::to_string(&config) {
            Ok(s) => s,
            Err(e) => format!("error:{e}"),
        },
        Err(e) => format!("error:{e}"),
    }
}

#[tauri::command]
fn save_config(config: String) -> String {
    match modsyncnext2::save_config(config) {
        Ok(()) => "ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn get_minecraft_versions() -> Vec<String> {
    modsyncnext2::get_minecraft_versions()
}

#[tauri::command]
fn get_mods_info(version: String) -> Vec<FileHashInfo> {
    modsyncnext2::get_mods_info(version)
}

fn main() {
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_config,
            save_config,
            has_config,
            create_default_config,
            choose_file,
            get_minecraft_versions,
            get_mods_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
