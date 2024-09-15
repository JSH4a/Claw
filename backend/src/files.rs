use crate::FileInfo;
use regex::Regex;
use std::fs;
use std::path::MAIN_SEPARATOR;
use std::time::SystemTime;

fn get_search_parts(search_string: &str) -> Vec<&str> {
    search_string.split(MAIN_SEPARATOR)
        .filter(|x| {!x.is_empty()})
        .collect::<Vec<&str>>()
}

fn matches_regex(string: &str, reg: &str) -> bool {
    match Regex::new(reg) {
        Ok(regex) => {
            regex.is_match(string)
        }
        Err(_) => {
            false
        }
    }
}

pub fn read_directory(directory_path: &str) -> Vec<FileInfo> {
    let mut file_list = Vec::new();
    let paths = fs::read_dir(directory_path).unwrap();

    for entry in paths {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        let file_path = entry.path()
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

    file_list
}

pub fn resolve_search(search_string: &str) -> String {
    let search_parts = get_search_parts(search_string);

    let mut results = read_directory(&("/".to_owned()));
    if search_parts.len() == 0 {
        return serde_json::to_string(&results).unwrap()
    }

    // Do the first match for the root read_directory
    results = results.into_iter()
        .filter(|result| matches_regex(&result.name, search_parts[0]))
        .collect();

    // Do the rest of the searching
    for part in &search_parts[1..] {
        results = results.into_iter()
            .flat_map(|result| read_directory(&result.path))
            .filter(|result| matches_regex(&result.name, part))
            .collect();
    }

    // if single directory, open it and return the results
    if results.len() == 1 && search_string == results[0].path {
        results = read_directory(&results[0].path);
    }

    // Serialize the list of file info to a JSON string
    serde_json::to_string(&results).unwrap()
}
