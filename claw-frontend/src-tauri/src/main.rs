// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
  // Get a list of all entries in the folder
  let entries = fs::read_dir(path)?;

  // Extract the filenames from the directory entries and store them in a vector
  let file_names: Vec<String> = entries
      .filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() {
          path.file_name()?.to_str().map(|s| s.to_owned())
        } else {
          None
        }
      })
      .collect();

  Ok(file_names)
}


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}