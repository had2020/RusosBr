use reqwest::Error;
use TerimalRtdm::*;

#[tokio::main]
async fn fetch_page(url: &str) -> String {
    // "https://example.com"
    // GET
    let client = reqwest::Client::new();
    let response = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
            .header("Accept", "text/html")
            //.header("Accept-Encoding", "gzip, deflate") //compressed response
            .send()
            .await.unwrap();

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
    let mut parsed = vec![String::new()];
    let mut inside_tag: bool = false;
    let mut inside_braces: bool = false;
    let mut element_type = false;
    let mut current_element_index = parsed.len() - 1;
    let mut char_rule_counter = 0;
    let mut correctly_closed: bool = false;

    for (_index, ch) in html_code.chars().enumerate() {
        let condition = (ch, inside_tag, element_type, inside_braces);

        match condition {
            ('>', true, true, false) => {
                if correctly_closed {
                    parsed.push(String::new());
                }
                correctly_closed = false;
                element_type = false;
                inside_tag = false;
                current_element_index += 1;
                inside_braces = false;
                char_rule_counter = 0;
            }
            ('>', true, true, true) => {
                // inside start
                correctly_closed = true;
                inside_braces = false;
                char_rule_counter = 0;
            }
            ('>', true, false, true) => {
                inside_braces = false;
                inside_tag = false;
                char_rule_counter = 0;
            }
            ('<', false, false, false) => {
                inside_tag = true;
                inside_braces = true;
                char_rule_counter = 0;
            }
            /*
            ('<', true, true, false) => {
                // cutt of before printing rest
                if correctly_closed {
                    correctly_closed = false;
                }
            }
            */
            ('p', true, false, true) | ('h', true, false, true) => {
                if char_rule_counter <= 2 && inside_braces {
                    element_type = true;
                }
            }
            _ => {
                if inside_tag && element_type && correctly_closed {
                    parsed[current_element_index].push(ch);
                } else {
                    char_rule_counter += 1;
                }
            }
        }
    }

    parsed
}

fn main() {
    clear();
    let mut app = App::new();

    let mut typed_text: String = String::new();
    let mut is_typing: bool = false;

    raw_line(": then q <- (Quit)");
    raw_line("e <- (Search bar)");
    raw_line("1 <- (example.com)");

    raw_mode(true);

    // app loop
    loop {
        clear(); // clear last loop, or Rust debug logs
        collect_presses(&mut app);

        if key_press(&app, ":") {
            if halt_press_check(&mut app, "q") {
                clear();
                break;
                //std::process::exit(1);
            }
        }

        if key_press(&app, "1") {
            let html_code = fetch_page("https://example.com");
            let parsed = parse_html_content(html_code);
            for (index, item) in parsed.iter().enumerate() {
                line(Position { x: index, y: 0 }, item, "yellow");
            }
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
