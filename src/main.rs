use chrono::Local;
use dotenv::dotenv;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::time::delay_for;
use std::{env, time::Duration};


#[tokio::main]
async fn main() {
    dotenv().ok();
    let current: u16 = env::var("CURRENT_NEWS").unwrap().parse().unwrap();
    let delay: u64 = env::var("DELAY").unwrap().parse().unwrap();

    monitor(current, delay).await;
}

async fn monitor(current: u16, delay: u64) {
    let client = Client::new();
    let mut new = current + 1;
    loop {
        delay_for(Duration::from_millis(delay)).await;
        let url = format!("{}{}", "https://supremenewyork.com/news/", &new.to_string());
        let resp = client.get(&url).send().await.unwrap();
        if resp.status() != 404 {
            let text = &resp.text().await.unwrap();
            let doc = Html::parse_document(&text);
            let selector = Selector::parse("h2").unwrap();

            let joined = doc
                .select(&selector)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("");
            
            let time = Local::now().format("%T");
            println!("[{}] {}", time, joined);
            new = &new + 1;
        }
    }
}
