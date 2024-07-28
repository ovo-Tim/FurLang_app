use serde::{Serialize, Deserialize};
mod server_runner;
use server_runner::CommandRunner;
use tauri::{async_runtime::block_on, path::BaseDirectory, Manager};
use std::fs;
use directories::UserDirs;
use lazy_static::lazy_static;
use std::sync::Mutex;

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

lazy_static!{
    static ref RUNNER:Mutex<CommandRunner> = Mutex::new(CommandRunner::default());
}

#[tauri::command]
fn start_server(handle: tauri::AppHandle) -> Result<(), String>{
    let server_path = handle.path().resolve("server/main".to_owned()+EXTENSION, BaseDirectory::Resource);
    if server_path.is_err(){
        return Err("Failed to get server path".into());
    }
    if let Some(_path) = server_path.unwrap().to_str(){
        let mut runner = RUNNER.lock().unwrap();
        runner.start_server(_path.to_string())?;
    }else{
        return Err("Failed to get server path".into());
    }
    Ok(())
}

#[tauri::command]
fn get_state() -> Result<server_runner::CommandState, String>{
    if let Ok(mut runner) = RUNNER.lock(){
        Ok(block_on(runner.get_state())?)
    }else{
        Err("Failed to get state. Rust mutex is poisoned.".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config, start_server, get_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
