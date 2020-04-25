use chrono::{Local, Utc};
use dotenv::dotenv;
use reqwest::Client;
use scraper::{Html, Selector};
use std::{env, time::Duration};
use tokio::time::delay_for;

mod discord;

use discord::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = Client::new();
    let delay: u64 = match env::var("DELAY") {
        Ok(delay) => delay,
        Err(_) => panic!("Could not get delay from env")
    }
    .parse()
    .unwrap();

    let mut newsid: u16 = match env::var("CURRENT_NEWS") {
        Ok(id) => id,
        Err(_) => panic!("Could not get current news id from env"),
    }
    .parse()
    .unwrap();

    loop {
        delay_for(Duration::from_millis(delay)).await;
        match monitor(newsid, &client).await {
            Ok(num) => newsid = num,
            Err(why) => println!("[{}] {}", Local::now().format("%T"), why),
        };
    }
}

async fn monitor(current: u16, client: &Client) -> Result<u16, String> {
    let time = Local::now().format("%T");
    let url = format!(
        "{}{}",
        "https://www.supremenewyork.com/news/",
        &current.to_string()
    );
    let resp = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("{}: {}", "Could not make request".to_string(), e)),
    };
    if resp.status() != 404 {
        let text = &resp.text().await.unwrap();
        let doc = Html::parse_document(&text);
        let title_selector = Selector::parse("h2").unwrap();
        let blurb_selector = Selector::parse(".blurb").unwrap();
        let img_selector = Selector::parse("img").unwrap();

        let title = match doc.select(&title_selector).next() {
            Some(element) => element,
            None => return Err("Error finding title".to_string()),
        }
        .text()
        .collect::<String>();

        let blurb = match doc.select(&blurb_selector).next() {
            Some(element) => element,
            None => return Err("Error finding blurb".to_string()),
        }
        .text()
        .collect::<String>();

        let img = match doc.select(&img_selector).next() {
            Some(element) => element,
            None => return Err("Error finding image".to_string()),
        }
        .value()
        .attr("src")
        .unwrap();

        println!("[{}] {}", time, &title);

        let embed = Embed {
            title,
            description: blurb,
            url,
            timestamp: Utc::now().format("%+").to_string(),
            color: 0xFFFFF,
            thumbnail: EmbedThumbnail {
                url: format!("https:{}", img),
            },
            footer: EmbedFooter {
                text: "Supreme News".to_owned(),
            },
        };

        match embed
            .send_discord(client, env::var("DISCORD_WEBHOOK").unwrap())
            .await
        {
            Ok(_) => (),
            Err(why) => return Err(format!("Could not send embed: {}", why)),
        }

        return Ok(current + 1);
    } else {
        return Ok(current);
    }
}
