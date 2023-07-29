use serde::Deserialize;

#[derive(Deserialize)]
pub struct Photo {
    pub identifier: String,
    md5: String,
    #[serde(rename(deserialize = "orderInEntry"))]
    pub order_in_entry: u32,
    #[serde(rename(deserialize = "type"))]
    file_type: String,
}

impl Photo {
    pub fn file_name(&self) -> String {
        format!("{}.{}", self.md5, self.file_type)
    }
}
