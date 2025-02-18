use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn does_directory_exist(directory_path: &str) -> bool {
  let path = PathBuf::from(directory_path);
  std::path::Path::new(&path).exists()
}

pub fn remove_directory_all(directory_path: &str) -> Result<(), std::io::Error> {
  let path = PathBuf::from(directory_path);
  fs::remove_dir_all(path)
}

//directory
pub fn create_directory(directory_path: &str) -> Result<(), std::io::Error> {
  let path = PathBuf::from(directory_path);
  fs::create_dir(path)
}

pub fn remove_directory(directory_path: &str) -> Result<(), std::io::Error> {
  let path = PathBuf::from(directory_path);
  fs::remove_dir_all(path)
}