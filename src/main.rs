mod scraper;

use scraper::{fetch_url, parse_html};
use futures::future::join_all;
use std::time::Instant;
use tokio::task;
use csv::Writer;
use std::fs::File;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org/",
        "https://www.mozilla.org/",
        "https://www.github.com/",
        "https://www.wikipedia.org/",
        "https://www.coursera.org/",
        "https://www.edureka.co/",
    ];

    let start = Instant::now();

    // Create a CSV writer
    let file = File::create("output.csv").expect("Unable to create file");
    let mut wtr = Writer::from_writer(file);

    // Write headers to the CSV file
    wtr.write_record(&["URL", "Title", "Meta Description", "Headings", "Links"]).expect("Unable to write headers");

    // Spawn tasks for each URL
    let tasks: Vec<_> = urls.into_iter().map(|url| {
        task::spawn(async move {
            match fetch_url(&url).await {
                Ok(body) => {
                    let (title, meta_description, headings, links) = parse_html(&url, &body);
                    let headings_str = headings.join(", ");
                    let links_str = links.join(", ");
                    (url, title, meta_description, headings_str, links_str)
                },
                Err(e) => {
                    eprintln!("Error fetching {}: {}", url, e);
                    (url, String::new(), String::new(), String::new(), String::new())
                },
            }
        })
    }).collect();

    // Await all tasks and collect results
    let results = join_all(tasks).await;

    // Write results to the CSV file
    for result in results {
        let (url, title, meta_description, headings_str, links_str) = result.expect("Task failed");
        wtr.write_record(&[&url, title.as_str(), meta_description.as_str(), headings_str.as_str(), links_str.as_str()]).expect("Unable to write record");
    }

    // Flush the CSV writer to ensure all data is written to the file
    wtr.flush().expect("Unable to flush writer");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
