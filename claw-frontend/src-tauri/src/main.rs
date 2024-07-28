// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn test(path: &str) -> String {
    format!("Hello, {}!", path)
}

#[tauri::command]
fn read_directory(directoryPath: &str) -> String {
    let paths = fs::read_dir(directoryPath).unwrap();

    let mut file_list = String::new();

    for path in paths {
        // do something
        let path = path.unwrap();

        if let Some(file_name) = path.file_name().to_str() {
            file_list.push_str(file_name);
            file_list.push('\n'); // Add a newline after each file name.
        }
    }

    // return here
    return format!("{}", file_list);
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .invoke_handler(tauri::generate_handler![read_directory])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}