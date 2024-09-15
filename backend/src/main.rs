// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod files;

use serde::Serialize;
use crate::files::{resolve_search,read_directory};
use std::fs;

#[derive(Serialize)]
// #[derive(Iterator)]
#[derive(Debug)]
struct FileInfo {
    name: String,
    path: String,
    file_type: String,
    last_modified: String,
}

#[tauri::command(rename_all = "snake_case")]
fn search_filesystem(path: &str) -> String {
    // If we 'search' for a directory and end with a '/' we should try to open it instead
    if path.ends_with('/') {
        match fs::canonicalize(path) {
            // If the path exists and is a directory we open it, otherwise we search as normal
            Ok(out) => {
                if out.metadata().unwrap().is_dir() {
                    return serde_json::to_string(&read_directory(path)).unwrap()
                }
            }
            Err(_) => {}
        }
    }
    resolve_search(path)
}

#[tauri::command(rename_all = "snake_case")]
fn open_directory(directory_path: &str) -> String {
    serde_json::to_string(&read_directory(directory_path)).unwrap()
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![search_filesystem,open_directory])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
