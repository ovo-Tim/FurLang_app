use serde::{Serialize, Deserialize};
use std::fs;
use directories::UserDirs;

#[derive(Serialize, Deserialize)]
struct Config {
    db_path: String,
    excluded_words_path: String,
    statistic_path: String,
    port: u8,
    dicts_path: String,
}

#[tauri::command]
fn get_config() -> Result<Config, String>{
    let user_dirs = UserDirs::new().unwrap();
    let config_path = user_dirs.home_dir().join("config.json");
    if let Ok(data) = fs::read_to_string(config_path) {
        if let Ok(conf) = serde_json::from_str(&data) {
            return Ok(conf);
        }
    }
    Err("Failed to read config file".into())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
