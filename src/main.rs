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

fn main() {
    clear();
    let mut app = App::new(5, 5);

    raw_line("q <- to quit");

    raw_mode(true);
    loop {
        clear();
        // app loop
        if key_pressed(&mut app, "q") {
            clear();
            break;
        }

        if key_pressed(&mut app, "w") {
            line(Position { x: 0, y: 5 }, "First", "blue");
            line(Position { x: 0, y: 11 }, "Sec", "red");
        }
    }

    raw_mode(false);
}
