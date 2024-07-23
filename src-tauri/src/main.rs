// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use modsyncnext2::read_config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_config() -> String {
    match read_config() {
        Ok(config) => match serde_json::to_string(&config) {
            Ok(s) => s,
            Err(e) => format!("error:{e}"),
        },
        Err(e) => format!("error:{e}"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
