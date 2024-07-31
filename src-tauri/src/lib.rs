use serde::{Serialize, Deserialize};
mod server_runner;
use server_runner::CommandRunner;
use tauri::{path::BaseDirectory, Manager};
use std::fs;
use directories::UserDirs;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod test;

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
    static ref RUNNER:Mutex<CommandRunner> = Mutex::new(CommandRunner::new());
}

#[tauri::command]
fn start_server(handle: tauri::AppHandle) -> Result<(), String>{
    let exe_path = handle.path().resolve("server/main".to_owned()+EXTENSION, BaseDirectory::Resource);
    if exe_path.is_err(){
        return Err("Failed to get server path".into());
    }
    let exe_path = exe_path.unwrap();
    println!("Server path: {:?}", exe_path);
    let mut runner = RUNNER.lock().unwrap();
    runner.start_server(exe_path)?;
    Ok(())
}

#[tauri::command]
fn get_state() -> Result<server_runner::CommandState, String>{
    if let Ok(mut runner) = RUNNER.lock(){
        let res = tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(runner.get_state())
        });
        Ok(res?)
    }else{
        Err("Failed to get state. Rust mutex is poisoned.".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn _run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config, start_server, get_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run(){
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(5)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { _run() });
}