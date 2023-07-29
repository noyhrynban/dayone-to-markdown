use rayon::prelude::*;
use serde::Deserialize;
use std::{
    fs::{self},
    path::Path,
    process::exit,
};

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Entry {
    text: String,
    creationDate: String,
    timeZone: Option<String>,
}

#[derive(Deserialize)]
struct Journal {
    entries: Vec<Entry>,
    // TODO: add audios. (Assume .m4a)
    // TODO: add photos. (Assume .jpeg ???)
}

fn main() {
    // TODO: refactor out parsing into function for cleaner/shorter main()
    let args: Vec<String> = std::env::args().collect();
    let arg = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No path was provided. Please provide a path to the journal JSON file.");
            exit(1)
        }
    };
    let journal_dir = Path::new(arg);
    let journal_json_file: &Path = &journal_dir.join("Journal.json");
    if !journal_json_file.exists() {
        println!("Path {} does not exist.", arg);
        exit(1)
    }

    let contents: String = match fs::read_to_string(journal_json_file) {
        Ok(contents) => contents,
        Err(e) => panic!("{e}"),
    };

    let journal: Journal = match serde_json::from_str(&contents) {
        Ok(journal) => journal,
        Err(e) => panic!("{e}"),
    };

    let new_journal_dir = Path::new("new_journal");

    journal.entries.par_iter().for_each(|entry| { // It is probably overkill to use Rayon 😛

        fs::create_dir_all(
            new_journal_dir
                .join(format!("{}", local_datetime.year()))
                .join(format!("{:02}", local_datetime.month()))
                .join(format!("{:02}", local_datetime.day())),
        );
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

// TODO: If multiple entries on same day, increment file name: entry-1.md entry-2.md, etc
