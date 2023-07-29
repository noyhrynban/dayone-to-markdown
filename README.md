# Day One to Markdown

Convert the JSON export from Day One into directories of Markdown entries with photos and audio files.

## Organization

### Entries

Entries are organized by date: `YYYY/MM/DD` with entries being named `entry.md`, `entry2.md`, etc to account for multiple entries on the same day.

### Media files

Photos and audio files are saved next to the entries to which they belong. In the export from Day One, photos are saved as `photos/<md5>.jpeg`. Audio files are similar but are `audios/<md5\>.m4a`. *(At least for me, all of the photos were JPEG and the audio recordings were M4A)* I have kept the media file names the same in order to avoid complexity of handling multiple entries with media on the same day.

## Prerequisite

[Install Rust](https://www.rust-lang.org/tools/install) to build locally.

## Usage

1. Use the JSON export in Day One to get a journal.zip file
2. Expand the .zip file
3. run with the path to the directory of the expanded archive
    - `cargo run -- ~/Desktop/journal`

This will create a directory `markdown_journal` that should look something like this:

```txt
markdown_journal
└── 2023
    └── 07
        └── 29
            ├── entry.md
            ├── entry2.md
            ├── <md5>.jpeg
            ├── <md5>.jpeg
            └── <md5>.m4a
```
