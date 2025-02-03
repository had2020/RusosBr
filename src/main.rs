use TerimalRtdm::*;

/*
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // url
    let url = "https://example.com";

    // GET
    let response = reqwest::get(url).await?;

    // vaildate success
    if response.status().is_success() {
        // get html content
        let html_content = response.text().await?;
        println!("HTML Content:\n{}", html_content);
    } else {
        eprintln!("Failed to fetch URL: {}", response.status());
    }

    Ok(())
}
*/

use std::io::{self, Read, Write};

fn main() {
    let mut app = App::new();

    line("q <- to quit");

    raw_mode(true);
    loop {
        if key_pressed(&mut app, "down") {
            break;
        }
    }
    raw_mode(false);
}
