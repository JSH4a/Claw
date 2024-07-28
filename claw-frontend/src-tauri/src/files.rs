use std::fs;
fn file_stuff() {
    let paths = fs::read_dir("/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

#[tauri::command]
fn read_directory(directory_path: &str) -> String {
    let paths = fs::read_dir(directory_path).unwrap();

    for path in paths {
      // do something
        let path = path.unwrap();

        if let Some(file_name) = path.file_name().to_str() {
            file_list.push_str(file_name);
            file_list.push('\n'); // Add a newline after each file name.
        }
    }

    // return here
    file_list;
}

fn main() {
    file_stuff();
}