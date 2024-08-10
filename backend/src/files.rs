use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::{files, FileInfo};

#[derive(Debug, Eq, PartialEq)]
enum SearchPartType {
    PATH,
    REGEX,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SearchPart {
    content: String,
    part_type: SearchPartType,
}

/// Splits the input string into sections of regex and non-regex parts.
pub fn split_into_regex_and_nonregex(input: &str) -> Vec<SearchPart> {
    let re = Regex::new(r"(\{.*?\})").unwrap();
    let mut result = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(input) {
        let start = mat.start();
        let end = mat.end();

        // Push the text before the curly braces.
        if last_end < start {
            result.push(SearchPart {
                content: input[last_end..start].to_string(),
                part_type: SearchPartType::PATH
            });
        }

        // Push the text inside the curly braces.
        result.push(SearchPart {
            content: input[start..end].to_string(),
            part_type: SearchPartType::REGEX
        });

        last_end = end;
    }

    // Push the remaining text after the last match.
    if last_end < input.len() {
        result.push(SearchPart {
            content: input[last_end..].to_string(),
            part_type: SearchPartType::PATH
        });
    }

    result
}

pub fn resolve_search(search_string: &str) -> Vec<String> {
    println!("New search");
    let search_parts = split_into_regex_and_nonregex(search_string);

    let mut results = Vec::new();

    for part in search_parts {
        if part.part_type == SearchPartType::PATH {
            let children = read_children_from_directory(part.content);
            results = concatenate_results_and_children(results, children);
        } else if part.part_type == SearchPartType::REGEX {
            results = filter_by_regex(&part.content, results);
        }

        println!("{:?}", results);
    }

    results
}

fn read_children_from_directory_old(directory_path: String) -> Vec<String> {
    match fs::read_dir(directory_path) {
        Ok(paths) => paths
            .filter_map(|entry| {
                entry.ok()
                    .and_then(|e| e.path().canonicalize().ok())
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn read_children_from_directory(dir: &str) -> Vec<String> {
    files::resolve_search(&directory_path);

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
    }t
}

fn concatenate_results_and_children(results: Vec<String>, children: Vec<String>) -> Vec<String> {
    if results.is_empty() {
        return children;
    }

    let mut concatenated = Vec::new();

    for result in &results {
        for child in &children {
            concatenated.push(format!("{}{}", result, child));
        }
    }

    concatenated
}

fn filter_by_regex(pattern: &str, strings: Vec<String>) -> Vec<String> {
    // Remove curly braces from the pattern
    let trimmed_pattern = pattern.trim_matches(|c| c == '{' || c == '}');

    // Compile the regex pattern
    let regex = match Regex::new(trimmed_pattern) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid regex pattern");
            return Vec::new(); // Return an empty vector if the pattern is invalid
        }
    };

    // Filter the strings based on the regex applied to the last segment
    strings.into_iter()
        .filter(|s| {
            // Split the string by '/'
            let segments: Vec<&str> = s.rsplit('/').collect();
            // Get the last segment (which was the first segment in the reversed vector)
            if let Some(last_segment) = segments.get(0) {
                // Apply the regex to the last segment
                regex.is_match(last_segment)
            } else {
                false
            }
        })
        .collect()
}

fn filter_by_regex_old(pattern: &str, strings: Vec<String>) -> Vec<String> {
    // Remove curly braces from the pattern
    let trimmed_pattern = pattern.trim_matches(|c| c == '{' || c == '}');

    // Compile the regex pattern
    let regex = match Regex::new(trimmed_pattern) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Invalid regex pattern");
            return Vec::new(); // Return an empty vector if the pattern is invalid
        }
    };

    // Filter the strings based on the regex
    strings.into_iter()
        .filter(|s| regex.is_match(s))
        .collect()
}

fn main() {
    println!("{:?}", split_into_regex_and_nonregex("/directory1/{*.txt}"));
}