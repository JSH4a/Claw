// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod files;

use std::time::{SystemTime};
use serde::Serialize;
use std::{fs};

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
    file_type: String,
    last_modified: String,
}

#[tauri::command(rename_all = "snake_case")]
fn read_directory(directory_path: &str) -> String {
    match fs::read_dir(directory_path) {
        Ok(paths) => {
            let mut file_list = Vec::new();

            for entry in paths {
                let entry = entry.unwrap();
                let metadata = entry.metadata().unwrap();

                let file_path = entry.path()
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                // Get file name
                let file_name = entry.file_name().into_string().unwrap();

                // Determine file type
                let file_type = if metadata.is_file() {
                    "File"
                } else if metadata.is_dir() {
                    "Directory"
                } else {
                    "Other"
                }.to_string();

                // Get last modified time
                let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let datetime: chrono::DateTime<chrono::Utc> = modified_time.into();
                let last_modified = datetime.format("%d/%m/%Y").to_string();

                // Create file info struct
                let file_info = FileInfo {
                    name: file_name,
                    path: file_path,
                    file_type,
                    last_modified,
                };

                // Add the file info to the list
                file_list.push(file_info);
            }

            // Serialize the list of file info to a JSON string
            serde_json::to_string(&file_list).unwrap()
        }
        Err(_) => {
            // Return a default string if the directory can't be read
            "{}".to_string()
        }
    }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![read_directory])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}