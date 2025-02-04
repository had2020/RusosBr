use reqwest::Error;
use TerimalRtdm::*;

#[tokio::main]
async fn fetch_page(url: &str) -> String {
    // "https://example.com"
    // GET
    let response = reqwest::get(url).await.unwrap();

    if response.status().is_success() {
        // get html content
        let html_content = response.text().await.unwrap();
        //println!("HTML Content:\n{}", html_content); // Debug
        html_content
    } else {
        eprintln!("Failed to fetch URL: {}", response.status());
        String::new()
    }
}

fn main() {
    clear();
    let mut app = App::new();

    let mut typed_text: String = String::new();
    let mut is_typing: bool = false;

    raw_line("q <- (Quit)");
    raw_line("e <- (Search bar)");
    raw_line("w <- (google.com)");

    raw_mode(true);

    // app loop
    loop {
        clear(); // clear last loop, or Rust debug logs
        collect_presses(&mut app);

        if key_press(&app, "q") {
            clear();
            break;
        }

        if key_press(&app, "w") {
            fetch_page("https://example.com");
            line(Position { x: 0, y: 5 }, "First", "blue");
            line(Position { x: 0, y: 11 }, "Sec", "red");
        }
        if key_press(&app, "e") {
            is_typing = true;
        }

        if is_typing && key_press(&app, "Enter") {
            is_typing = false;
            line(Position { x: 0, y: 1 }, &typed_text, "red");
        }

        use std::io::{self, Read, Write};

        if is_typing {
            let bytes_read = io::stdin().read(&mut app.key_buffer).unwrap();
            println!("{:?}", &app.key_buffer[..bytes_read]);
        }

        if is_typing {
            if app.keys_pressed.len() == 1 {
                typed_text = format!("{}{}", typed_text, app.keys_pressed);
            }
        }
    }

    raw_mode(false);
}
