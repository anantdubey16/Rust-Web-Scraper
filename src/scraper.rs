// src/scraper.rs

use reqwest::Error;
use select::document::Document;
use select::predicate::{Name, Attr};

pub async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub fn parse_html(url: &str, body: &str) {
    let document = Document::from(body);
    let title = document.find(Name("title")).next().map(|n| n.text()).unwrap_or_default();
    println!("URL: {}", url);
    println!("Title: {}", title);

    let meta_description = document.find(Attr("name", "description")).next()
        .and_then(|n| n.attr("content")).unwrap_or_default();
    println!("Meta Description: {}", meta_description);

    println!("Headings:");
    for heading in document.find(Name("h1")).chain(document.find(Name("h2"))).chain(document.find(Name("h3"))) {
        println!("  - {}", heading.text());
    }

    println!("Links:");
    for link in document.find(Name("a")).filter_map(|n| n.attr("href")) {
        println!("  - {}", link);
    }
}
