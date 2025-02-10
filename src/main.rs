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
            .header("Accept-Encoding", "gzip, deflate") //compressed response
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
    //let mut inner_content = String::new();

    /*
    if let Some(first_index) = html_code.find("<h1>") {
        if let Some(second_index) = html_code.find("</h1>") {
            let start_index = first_index + "<h1>".len();

            let chars_num = second_index - start_index;

            for iter in 0..chars_num {
                let char = html_code.chars().nth(start_index + iter).unwrap();
                inner_content = format!("{}{}", inner_content, char);
            }
            parsed.push(inner_content.clone());
        }
    }

    if let Some(first_index) = html_code.find("<p>") {
        if let Some(second_index) = html_code.find("</p>") {
            let start_index = first_index + "<p>".len();

            let chars_num = second_index - start_index;

            for iter in 0..chars_num {
                let char = html_code.chars().nth(start_index + iter).unwrap();
                inner_content = format!("{}{}", inner_content, char);
            }
            parsed.push(inner_content.clone());
        }
    }
    */
    struct Element {
        fi: usize, // first index
        si: usize, // second index
    }

    let mut current_element = Element { fi: 0, si: 0 };
    let mut inside_tag: bool = false;
    let mut element_type = 0; // 1 is normal text: p or h1-h6
    let mut current_element_index = 0;

    for (index, ch) in html_code.chars().enumerate() {
        let condition = (ch, inside_tag);

        match condition {
            ('<', true) => {
                inside_tag = false;
                current_element_index += 1;
            }
            ('>', false) => inside_tag = true,
            ('p', false) | ('h', false) => element_type = 1,
            inside_tag => parsed[current_element_index].push(ch),
            _ => (),
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
