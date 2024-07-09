mod scraper;

use scraper::{fetch_url, parse_html};
use futures::future::join_all;
use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org/",
        "https://www.mozilla.org/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
    ];

    let start = Instant::now();

    // Spawn tasks for each URL
    let tasks: Vec<_> = urls.into_iter().map(|url| {
        task::spawn(async move {
            match fetch_url(&url).await {
                Ok(body) => parse_html(&body),
                Err(e) => eprintln!("Error fetching {}: {}", url, e),
            }
        })
    }).collect();

    // Await all tasks
    join_all(tasks).await;

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
