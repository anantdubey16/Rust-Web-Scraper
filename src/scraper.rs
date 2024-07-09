// src/scraper.rs

use reqwest::Error;
use select::document::Document;
use select::predicate::Name;

pub async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub fn parse_html(body: &str) {
    let document = Document::from(body);
    for node in document.find(Name("title")) {
        println!("Title: {}", node.text());
    }
}
