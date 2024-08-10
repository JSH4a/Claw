use regex::Regex;
use std::{env, path};
use std::fs;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;
use std::time::SystemTime;
use serde_json::to_vec;
use crate::FileInfo;

fn get_search_parts(search_string: &str) -> Vec<&str> {
    search_string.split(MAIN_SEPARATOR)
        .filter(|x| {!x.is_empty()})
        .collect::<Vec<&str>>()
}

fn matches_regex(string: &str, regex: &str) -> bool {
    match Regex::new(regex) {
        Ok(regex) => {
            regex.is_match(string)
        }
        Err(_) => {
            false
        }
    }
}

fn read_directory(directory_path: &str) -> Vec<String> {
    let mut file_list = Vec::new();
    match fs::read_dir(directory_path) {
        Ok(paths) => {

            for entry in paths {
                let entry = entry.unwrap();

                let file_path = entry.path()
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .sto_string();

                // Add the file info to the list
                file_list.push(file_path);
            }

            // Serialize the list of file info to a JSON string
            file_list
        }
        Err(_) => {
            // Return a default string if the directory can't be read
            file_list
        }
    }
}

pub fn resolve_search(search_string: &str) {
    let search_parts = get_search_parts(search_string);

    let mut paths = read_directory("/");

    for part in search_parts {
        let children: Vec<String> = paths.iter().map(|x| { read_directory(x) }).flat_map(|x1| {x1}).collect();

        paths = children.iter().filter(|child| { matches_regex(child, part) }).collect();
    }

    println!("{:?}", paths);
}