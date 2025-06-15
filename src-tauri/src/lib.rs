// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


*/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::async_runtime;
use tauri::{AppHandle, Manager};
//use tauri::api::path;
use std::path::PathBuf;
/*
#[tauri::command]
fn send_message(message: String) -> String {
    format!("Echo: {}", message)
}
*/
////////////////////////////
use std::sync::Arc;
use tokio::sync::Mutex;

mod user_chats;

use crate::user_chats::user_chats::UserChats;

/*
// --- Commands ---

#[tauri::command]
async fn increment_count(state: tauri::State<'_, AppState>) -> Result<u32, String> {
    let mut count = state.count.lock().await;
    *count += 1;
    Ok(*count)
}

#[tauri::command]
async fn get_count(state: tauri::State<'_, AppState>) -> Result<u32, String> {
    let count = state.count.lock().await;
    Ok(*count)
}
*/
//////////////
//////////////
//////////////

mod frontend_request;
use crate::frontend_request::handle_frontend_request;

//////////////
/////////////////////////
mod app_state;
use crate::app_state::app_paths::get_app_data_dir;
use crate::app_state::app_paths::get_app_paths;
use crate::app_state::app_state::AppState;
///////////////////////////

mod content;
use crate::content::content::run_content_localhost;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data_path: PathBuf;

    match get_app_paths() {
        Some((config_path, data_path, cache_path)) => {
            //println!("Config dir: {}", config_path.display());
            //println!("Data dir: {}", data_path.display());
            //println!("Cache dir: {}", cache_path.display());
            println!("init user settings");
            app_data_path = data_path;
        }
        None => {
            println!("Could not determine project directories");
        }
    }

    let user_chats = UserChats::new();
    //

    //

    tauri::Builder::default()
        
        .setup(|app| {
            if let Some(data_dir) = get_app_data_dir() {
                println!("Data directory: {}", data_dir.display());
                tauri::async_runtime::spawn(async move {
                    run_content_localhost(data_dir).await;
                });
            } else {
                eprintln!("Failed to get data directory.");
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            //count: Mutex::new(0).into(), // .into() creates Arc<Mutex<_>>
            //shared_app_daemon_proxy: Mutex::new(app_daemon_proxy),
            shared_user_chats: Mutex::new(user_chats).into(),
        })
        .invoke_handler(tauri::generate_handler![
            //get_data_file_path,
            //get_cache_path_command,
            //increment_count, get_count,
            handle_frontend_request //send_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
