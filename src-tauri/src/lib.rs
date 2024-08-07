use serde::{Serialize, Deserialize};
mod server_runner;
use server_runner::CommandRunner;
use tauri::{path::BaseDirectory, Manager};
use std::{error::Error, fs};
use directories::UserDirs;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Config {
    db_path: String,
    excluded_words_path: String,
    statistic_path: String,
    port: u32,
    dicts_path: String,
}

#[tauri::command]
fn get_config() -> Result<Config, String>{
    let user_dirs = UserDirs::new().unwrap();
    let config_path = user_dirs.home_dir().join(".furlang/config.json");
    println!("Config path: {:?}", config_path);

    let file = match fs::read_to_string(config_path) {
        Ok(data) => data,
        Err(_) => return Err("Failed to read config file".into()),
    };

    let conf: Config = match serde_json::from_str(&file) {
        Ok(conf) => conf,
        Err(e) => return Err("Failed to parse config file: ".to_string()+&e.to_string()),
    };
    Ok(conf)
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
    runner.start_server(exe_path, None)?;
    Ok(())
}

#[tauri::command]
fn get_state() -> Result<server_runner::CommandState, String>{
    if let Ok(mut runner) = RUNNER.lock(){
        let res = runner.get_state();
        Ok(res?)
    }else{
        Err("Failed to get state. Rust mutex is poisoned.".into())
    }
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let main_window = app.get_webview_window("main").unwrap();
    main_window.on_window_event(|event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            RUNNER.lock().unwrap().kill();
        }
    });
    // main_window.set_shadow(true);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn _run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config, start_server, get_state])
        .setup(setup)
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