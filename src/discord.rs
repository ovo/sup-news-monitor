use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: String,
    pub timestamp: u64,
    pub color: u64,
    pub thumbnail: EmbedThumbnail,
    pub footer: EmbedFooter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
}

impl Embed {
    pub fn send_discord(&self) {
        println!("{:?}", &self);
    }
}
