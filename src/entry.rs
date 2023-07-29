use chrono::{DateTime, FixedOffset};
use chrono_tz::Tz;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Entry {
    text: String,
    creationDate: String,
    timeZone: Option<String>,
}

impl Entry {
    pub fn text(&self) -> String {
        self.text.cleanup()
    }

    pub fn local_datetime(&self) -> DateTime<Tz> {
        let date: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(&self.creationDate) {
            Ok(dt) => dt,
            Err(e) => panic!("{e}"),
        };
        let timezone = match &self.timeZone {
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
    fn replace_dayone_photo(&self) -> String;
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
    }

    fn replace_dayone_photo(&self) -> String {
        /*
        replace
        ![](dayone-moment://CF3AECB79C3247E8B9D7A7F86ACCC76E)
        with
        ![](image://CF3AECB79C3247E8B9D7A7F86ACCC76E.jpeg)

        In the entry there is a list of audios and or photos.
        Each audio/photo in that list has 'identifier' that matches what is in the dayone-moment.
        the value of 'md5' for the audio/photo entry is the body of the file name
        -> journal/audios/<md5>.m4a
        -> journal/photos/<md5>.jpeg
         */
        self.to_string()
    }
}
