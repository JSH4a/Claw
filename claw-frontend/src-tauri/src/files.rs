use std::{fs, io};
fn file_stuff() {
    let paths = fs::read_dir("/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
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