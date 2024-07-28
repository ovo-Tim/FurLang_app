use serde::{Serialize, Deserialize};
mod server_runner;
use server_runner::CommandRunner;
use tauri::{path::BaseDirectory, Manager};
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

#[cfg(target_os = "windows")]
static EXTENSION: &str = ".exe";

#[cfg(not(target_os = "windows"))]
static EXTENSION: &str = ".bin";

fn start_server(handle: tauri::AppHandle, runner: &mut CommandRunner) -> Result<(), String>{
    let server_path = handle.path().resolve("server/main".to_owned()+EXTENSION, BaseDirectory::Resource);
    if server_path.is_err(){
        return Err("Failed to get server path".into());
    }
    if let Some(_path) = server_path.unwrap().to_str(){
        runner.start_server(_path.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut runner = CommandRunner::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
