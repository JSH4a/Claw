// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod files;

use std::time::SystemTime;
use serde::Serialize;
use std::fs;
use crate::files::resolve_search;

#[derive(Serialize)]
enum FileType {
    Directory,
    File,
    Link,
    BrokenLink,
    Other,
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
    file_type: FileType,
    last_modified: String,
}

#[tauri::command(rename_all = "snake_case")]
fn read_directory(directory_path: &str) -> String {
    resolve_search(directory_path);
    match fs::read_dir(directory_path) {
        Ok(paths) => {
            let mut file_list = Vec::new();

            for entry in paths {
                let entry = entry.unwrap();
                let metadata = entry.metadata().unwrap();
                let f_type = metadata.file_type();


                // Get file name, if we fail to turn file name into string (contains non unicode
                // data) then skip that file
                let file_name = match entry.file_name().into_string() {
                    Ok(s) => { s }
                    Err(_) => { continue; }
                };

                // Determine file type
                let mut file_type = if f_type.is_file() {
                    FileType::File
                } else if f_type.is_dir() {
                    FileType::Directory
                } else if f_type.is_symlink() {
                    FileType::Link
                } else {
                    FileType::Other
                };

                let file_path = match entry.path().canonicalize() {
                    Ok(p) => { p.to_str().unwrap().to_string() }
                    Err(_) => {
                        file_type = FileType::BrokenLink;
                        directory_path.to_string()
                    }
                };

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

#[tauri::command(rename_all = "snake_case")]
fn open_file(file_path: &str) -> String {
    match open::that(file_path) {
        Ok(_) => {
            "".to_string()
        }
        Err(e) => {
            e.to_string()
        }
    }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![read_directory, open_file])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
