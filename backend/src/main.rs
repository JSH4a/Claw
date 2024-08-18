// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod files;

use std::time::{SystemTime};
use serde::Serialize;
use std::{fs};
use crate::files::resolve_search;

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
fn read_directory(directory_path: &str) -> String {
    resolve_search(directory_path)
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![read_directory])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}