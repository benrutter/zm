use dirs::home_dir;
use std::env;
use std::fs;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args: Vec<String> = env::args().collect();
    let to_match: &String = &args[1];
    if let Some(matched_path) = match_in_dir(&to_match, "./") {
        println!("{}", matched_path);
    } else if let Some(matched_path) = match_in_dir_recursive(&to_match, "./") {
        println!("{}", matched_path);
    } else if let Some(matched_path) =
        match_in_dir_recursive(&to_match, home_dir().expect("REASON").to_str().unwrap())
    {
        println!("{}", matched_path);
    } else {
        println!("{}", to_match);
    }
}

fn match_in_dir(to_match: &str, dir: &str) -> Option<String> {
    for entry in fs::read_dir(dir).ok()? {
        let ok_entry = entry.ok()?;
        if let Some(entry_str) = ok_entry.path().to_str() {
            if entry_str.ends_with(to_match) && ok_entry.path().is_dir() {
                return Some(entry_str.to_string());
            }
        }
    }
    None
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn match_in_dir_recursive(to_match: &str, dir: &str) -> Option<String> {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        if let Some(entry_str) = entry.path().to_str() {
            if entry_str.ends_with(to_match) && entry.path().is_dir() {
                return Some(entry_str.to_string());
            }
        }
    }
    None
}
