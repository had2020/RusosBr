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

fn parse_html_content(html_code: String) -> Vec<String> {
    vec![String::new()]
}

fn main() {
    clear();
    let mut app = App::new();

    let mut typed_text: String = String::new();
    let mut is_typing: bool = false;

    raw_line(": then q <- (Quit)");
    raw_line("e <- (Search bar)");
    raw_line("1 <- (google.com)");

    raw_mode(true);

    // app loop
    loop {
        clear(); // clear last loop, or Rust debug logs
        collect_presses(&mut app);

        if key_press(&app, ":") {
            if halt_press_check(&mut app, "q") {
                clear();
                break;
            }
        }

        if key_press(&app, "1") {
            let html_code = fetch_page("https://example.com");
            let parsed = parse_html_content(html_code);
            line(Position { x: 0, y: 5 }, "First", "white");
        }
        if key_press(&app, "e") && !is_typing {
            is_typing = true;
            app.keys_pressed = String::new();
        }

        if is_typing && key_press(&app, "Enter") {
            is_typing = false;
            line(Position { x: 0, y: 1 }, &typed_text, "red");
        }

        if is_typing && key_press(&app, "Space") {
            typed_text = format!("{} ", typed_text);
        }

        if is_typing && key_press(&app, "Backspace") {
            typed_text.pop();
        }

        if is_typing {
            if app.keys_pressed.len() == 1 {
                typed_text = format!("{}{}", typed_text, app.keys_pressed);
            }
        }

        if is_typing {
            line(Position { x: 0, y: 1 }, &typed_text, "blue");
        }
    }

    raw_mode(false);
}
