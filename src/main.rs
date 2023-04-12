use std::io::{self, Write};
use tokio::runtime::Runtime;
use reqwest::header;

fn main() {
    let mut input = String::new();
    print!("Enter a term to search on Wikipedia: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let query = input.trim();

    let url = format!("https://en.wikipedia.org/w/api.php?action=opensearch&search={}&format=json", query);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(header::USER_AGENT, "My Rust Wikipedia Searcher")
            .send().await.unwrap()
            .json::<serde_json::Value>().await.unwrap();

        let results = response[1].as_array().unwrap();
        let descriptions = response[2].as_array().unwrap();
        let links = response[3].as_array().unwrap();

        println!("Search results for '{}':", query);
        for i in 0..results.len() {
            println!("- Title: {}", results[i]);
            println!("  Description: {}", descriptions[i]);
            println!("  Link: {}", links[i]);
            println!();
        }
    });
}
