use std::error::Error;
use scraper::{Html, Selector};


// Create a function to get the repo ID
fn get_id(url: String) -> Result<String, Box<dyn Error>> {
    // Get the HTML from the URL
    let body = reqwest::blocking::get(&url)?.text()?;
    println!("URL is: {}", url);
    // Parse the HTML
    let document = Html::parse_document(&body);
    // Create a selector to find the repo ID which is found in a meta tag like this one
    // <meta name="octolytics-dimension-repository_id" content="635741999" />
    let selector = Selector::parse(r#"meta[name="octolytics-dimension-repository_id"]"#).unwrap();
    
    // Find the repo ID
    let id = document.select(&selector).next().unwrap().value().attr("content").unwrap().to_string();
    // Return the repo ID
    Ok(id)
}


fn create_url(repo_arg: &str) -> String {
    // Create the URL string
    let url = format!("https://github.com/{}", repo_arg);
    // Return the URL
    url
}


fn main() {
    // Get the last argument in the CLI
    let arg = std::env::args().last().unwrap();

    // if the last argument looks anything like help, print the help message
    if arg.contains("help") {
        println!("Usage: {} [string]", std::env::args().nth(0).unwrap());
        println!("\nUse a partial repository path like alfredodeza/rust-fundamentals to get the ID");
        return;
    }

    // Get the partial repo from the last argument
    let repo = create_url(&arg);
    // Get the repo ID
    let id = get_id(repo).unwrap();
    println!("Open in Codespaces badge\n[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo={})", id);
    println!("{}", id)

}
