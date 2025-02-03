use reqwest::Error;
use TerimalRtdm::*;

#[tokio::main]
async fn fetch_page() -> String {
    // url
    let url = "https://google.com"; // example & example.com

    // GET
    let response = reqwest::get(url).await.unwrap();

    // vaildate success
    if response.status().is_success() {
        // get html content
        let html_content = response.text().await.unwrap();
        //println!("HTML Content:\n{}", html_content);
        html_content
    } else {
        eprintln!("Failed to fetch URL: {}", response.status());
        String::new()
    }
}

fn main() {
    clear();
    let mut app = App::new();

    let mut all_presses: String = String::new();

    raw_line("q <- to quit");
    raw_line("w <- to show lines");

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
            line(Position { x: 0, y: 5 }, "First", "blue");
            line(Position { x: 0, y: 11 }, "Sec", "red");
        }
    }

    raw_mode(false);
}
