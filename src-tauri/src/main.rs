// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/
//////////////////
//////////////////
/*
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
*/
//use tauri::Manager;
//use tauri::command;

#[tauri::command]
fn send_message(message: String) -> String {
    let response = format!("Server received: {}", message);
    
    // Send back the response to the frontend
    response
}
//////////////////
//////////////////
use tauri::api::path::app_data_dir;
use std::path::PathBuf;

#[tauri::command]
fn get_image_path(file_name: String) -> Result<String, String> {
    let mut path = PathBuf::new();
    path.push(app_data_dir(&tauri::generate_context!().config()).unwrap()); // Pass the config directly
    path.push(file_name);
    
    println!("PATH OF THE FILE: {:}", path.to_string_lossy().into_owned()); 
    Ok(path.to_string_lossy().into_owned())
    /*
    if path.exists() {
        Ok(path.to_string_lossy().into_owned())
    } else {
        Err("File not found".to_string())
    }
    */
}
/////////////////
/////////////////

mod app_data;
use crate::app_data::app_paths::get_app_data_dir;
use crate::app_data::app_paths::get_app_cache_dir;

fn main() {
    let app_data_path = get_app_data_dir()
    .expect("Failed to access or create app data directory");

    println!("App data directory created or exists at: {:?}", app_data_path);

    
    let app_cache_path = get_app_cache_dir()
    .expect("Failed to access or create app cache directory");

    println!("App cache directory created or exists at: {:?}", app_cache_path);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_message, get_image_path]) // Add get_image_path here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/