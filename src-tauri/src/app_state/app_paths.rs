/*
use std::path::PathBuf;
use tauri::AppHandle;
use directories_next::ProjectDirs;

const APP_NAME: &str = "global";
const APP_CACHE_DIR: &str = "Cache";

pub fn get_app_data_dir(_app: &AppHandle) -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "your-org", APP_NAME)
        .ok_or("Could not determine platform directories")?;
    let path = proj_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn get_app_data_cached_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let base = get_app_data_dir(app)?;
    let cached = base.join(APP_CACHE_DIR);
    std::fs::create_dir_all(&cached).map_err(|e| e.to_string())?;
    Ok(cached)
}


use std::env;

pub fn get_app_current_exe_dir() -> Option<std::path::PathBuf> {
    env::current_exe().ok().and_then(|path| path.parent().map(|p| p.to_path_buf()))
}
*/

use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_app_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "com.global.app")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
}

pub fn get_app_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "com.global.app").map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

pub fn get_app_cache_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "com.global.app").map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

pub fn get_app_paths() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let config = get_app_config_dir()?;
    let data = get_app_data_dir()?;
    let cache = get_app_cache_dir()?;

    println!("Config dir: {}", config.display());
    println!("Data dir: {}", data.display());
    println!("Cache dir: {}", cache.display());

    Some((config, data, cache))
}

/*
use tauri::{AppHandle, Manager};
use directories::ProjectDirs;

pub fn get_user_dirs(app: &AppHandle) {
    let package_info = app.package_info();
    let name = package_info.name.clone();
    let version = package_info.version.clone();
    println!("App name: {}, version: {}", name, version);

    // You still need to hardcode domain/org (e.g., from tauri.conf)
    if let Some(proj_dirs) = ProjectDirs::from("com", "com.global.app", &name) {
        //println!("Config dir: {}", proj_dirs.config_dir().display());
        println!("Config dir: {}", proj_dirs.config_dir().display());
        println!("Data dir: {}", proj_dirs.data_dir().display());
        println!("Cache dir: {}", proj_dirs.cache_dir().display());
    }
}
*/
/*
ProjectDirs::data_dir()
This is meant specifically for storing non-configuration app data, like images, caches, documents, etc.

Windows: C:\Users\You\AppData\Roaming\YourCompany\YourApp\data

macOS: /Users/You/Library/Application Support/YourApp

Linux: /home/you/.local/share/YourApp

Perfect for storing images — it’s writeable, user-specific, and appropriate for app-generated media.

Others (Less Ideal for Images):
config_dir()
For config files only (settings, preferences).

Not recommended for storing media files like images.

cache_dir()
Temporary or reproducible data.

Can be cleared by the OS — not safe for images you want to keep.

*/
