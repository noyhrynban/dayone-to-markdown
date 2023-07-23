use serde::Deserialize;
use std::{
    fs::{self},
    path::Path,
    process::exit,
};

#[derive(Debug, Deserialize)]
struct Entry {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Journal {
    entries: Vec<Entry>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No path was provided. Please provide a path to the journal JSON file.");
            exit(1)
        }
    };
    let file_path: &Path = Path::new(arg);
    if !file_path.exists() {
        println!("Path does not exist.");
        exit(1)
    }

    let contents: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("{e}"),
    };

    let journal: Journal = match serde_json::from_str(&contents) {
        Ok(journal) => journal,
        Err(e) => panic!("{e}"),
    };
    for entry in journal.entries {
        println!("{}", cleanup(entry.text).as_str());
    }
}

fn cleanup(string: String) -> String {
    string
        .replace(r"\.", ".")
        .replace(r"\(", "(")
        .replace(r"\)", ")")
        .replace(r"\!", "!")
        .replace(r"\-", "-")
        .replace(r"\+", "+")
        .replace(r"\[", "[")
        .replace(r"\]", "]")
}
