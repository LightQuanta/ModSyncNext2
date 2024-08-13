// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_config,
            save_config,
            has_config,
            create_default_config,
            choose_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
