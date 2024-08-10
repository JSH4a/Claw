use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Splits the input string into sections of regex and non-regex parts.
fn split_into_regex_and_nonregex(input: &str) -> Vec<String> {
    // This regex identifies sections that could be regex or contain special characters.
    let re = Regex::new(r"\{.*?\}|\[.*?\]|\(.*?\)|/.*[^\w/].*").unwrap();

    let mut sections = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(input) {
        if mat.start() > last_end {
            // Push the non-regex section before the regex match.
            sections.push(input[last_end..mat.start()].to_string());
        }
        // Push the regex section.
        sections.push(mat.as_str().to_string());
        last_end = mat.end();
    }

    if last_end < input.len() {
        // Push any remaining non-regex section after the last match.
        sections.push(input[last_end..].to_string());
    }

    sections
}

fn main() {
    println!("{:?}", split_into_regex_and_nonregex("/directory1/{*.txt}"));
}