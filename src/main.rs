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
    raw_line("w <- (Search bar)");
    raw_line("e <- (google.com)");

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
        }

        if is_typing {
            typed_text = format!("{}{}", typed_text, app.keys_pressed);
        }
    }

    raw_mode(false);
}
