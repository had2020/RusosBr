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
    let mut app = App::new(5, 5);

    raw_line("q <- to quit");

    raw_mode(true);
    loop {
        // app loop
        if key_pressed(&mut app, "q") {
            break;
        }

        clear();
    }
    line(Position { x: 0, y: 5 }, "Hello", "blue");
    line(Position { x: 0, y: 10 }, "Hello", "red");

    raw_mode(false);
}
