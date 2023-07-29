use crate::photo::Photo;
use chrono::{DateTime, FixedOffset};
use chrono_tz::Tz;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    text: String,
    creation_date: String,
    pub photos: Option<Vec<Photo>>,
    time_zone: Option<String>,
}

impl Entry {
    pub fn text(&self) -> String {
        let mut text = self.text.cleanup();
        if let Some(photos) = &self.photos {
            for photo in photos {
                text = text.replace(&photo.identifier, photo.file_name().as_str())
            }
        }
        text
    }

    pub fn local_datetime(&self) -> DateTime<Tz> {
        let date: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(&self.creation_date) {
            Ok(dt) => dt,
            Err(e) => panic!("{e}"),
        };
        let timezone = match &self.time_zone {
            Some(timezone) => timezone,
            None => {
                println!("{}", self.text);
                panic!();
            }
        };
        let tz: Tz = match timezone.parse() {
            Ok(tz) => tz,
            Err(e) => {
                println!("{}", self.text);
                panic!("{e}")
            }
        };
        date.with_timezone(&tz)
    }
}

trait EntryText {
    fn cleanup(&self) -> String;
}

impl EntryText for String {
    fn cleanup(&self) -> String {
        self.replace(r"\.", ".")
            .replace(r"\(", "(")
            .replace(r"\)", ")")
            .replace(r"\!", "!")
            .replace(r"\-", "-")
            .replace(r"\+", "+")
            .replace(r"\[", "[")
            .replace(r"\]", "]")
            .replace(r"dayone-moment://", "")
    }
}
