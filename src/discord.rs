use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    embeds: [Embed; 1] 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: String,
    pub timestamp: String,
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
    pub async fn send_discord(self, client: &Client, webhook: String) -> Result<(), reqwest::Error> {
        let load = Payload {
            embeds: [self]
        };
        client
            .post(&webhook)
            .json(&load)
            .send()
            .await?;
        
        Ok(())
    }
}
