mod audio;
mod entry;
mod photo;

use chrono::Datelike;
use entry::Entry;
use rayon::prelude::*;
use serde::Deserialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Deserialize)]
struct Journal {
    entries: Vec<Entry>,
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

    let new_journal_dir = Path::new("markdown_journal");

    journal.entries.par_iter().for_each(|entry| {
        let local_datetime = entry.local_datetime();
        let entry_dir: &PathBuf = &new_journal_dir
            .join(format!("{}", local_datetime.year()))
            .join(format!("{:02}", local_datetime.month()))
            .join(format!("{:02}", local_datetime.day()));

        match fs::create_dir_all(entry_dir) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }

        if let Some(audios) = &entry.audios {
            audios.par_iter().for_each(|audio| {
                let audio_path = journal_dir.join("audios").join(audio.file_name());
                if !audio_path.exists() {
                    println!(
                        "File for audio in entry is missing: {}",
                        audio_path.to_string_lossy()
                    );
                } else {
                    match fs::copy(audio_path, entry_dir.join(audio.file_name())) {
                        Ok(_) => {}
                        Err(e) => panic!("{e}"),
                    };
                }
            });
        }

        if let Some(photos) = &entry.photos {
            photos.par_iter().for_each(|photo| {
                let photo_path = journal_dir.join("photos").join(photo.file_name());
                if !photo_path.exists() {
                    println!(
                        "File for photo in entry is missing: {}",
                        photo_path.to_string_lossy()
                    );
                } else {
                    match fs::copy(photo_path, entry_dir.join(photo.file_name())) {
                        Ok(_) => {}
                        Err(e) => panic!("{e}"),
                    };
                }
            });
        }

        let text: String = entry.text();
        let mut entry_num: usize = 1;
        let mut text_entry_path = entry_dir.join("entry.md");
        while text_entry_path.exists() {
            entry_num += 1; // reasonably safe assumption this won't roll over
            text_entry_path = entry_dir.join(format!("entry{}.md", entry_num));
        }
        match fs::write(text_entry_path, text) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        };
    });
}
