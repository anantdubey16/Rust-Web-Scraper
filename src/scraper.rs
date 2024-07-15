use reqwest::Error;
use select::document::Document;
use select::predicate::{Name, Attr};

pub async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub fn parse_html(url: &str, body: &str) -> (String, String, Vec<String>, Vec<String>) {
    let _ = url;
    let document = Document::from(body);
    let title = document.find(Name("title")).next().map(|n| n.text()).unwrap_or_default();

    let meta_description = document.find(Attr("name", "description")).next()
        .and_then(|n| n.attr("content")).unwrap_or_default().to_string();

    let mut headings = Vec::new();
    for heading in document.find(Name("h1")).chain(document.find(Name("h2"))).chain(document.find(Name("h3"))) {
        headings.push(heading.text());
    }

    let mut links = Vec::new();
    for link in document.find(Name("a")).filter_map(|n| n.attr("href")) {
        links.push(link.to_string());
    }

    (title, meta_description, headings, links)
}
