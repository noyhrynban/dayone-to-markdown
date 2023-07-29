use serde::Deserialize;

#[derive(Deserialize)]
pub struct Audio {
    pub identifier: String,
    md5: String,
    #[serde(rename(deserialize = "orderInEntry"))]
    pub order_in_entry: u32,
}

impl Audio {
    pub fn file_name(&self) -> String {
        format!("{}.m4a", self.md5)
    }
}
