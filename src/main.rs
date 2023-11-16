use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct ScraperError;

pub fn scrape_url(url: &str, xpath: &str) -> Result<String, ScraperError> {
    // Make an HTTP request to the URL
    let response = Client::new()
        .get(url)
        .send()
        .map_err(|_| ScraperError)?;

    // Check if the request was successful (status code 200)
    if !response.status().is_success() {
        return Err(ScraperError);
    }

    // Read the response body as a string
    let body = response.text().map_err(|_| ScraperError)?;

    // Parse the HTML using the scraper crate
    let document = Html::parse_document(&body);

    // Create a scraper selector from the provided XPath
    let selector = Selector::parse(xpath).map_err(|_| ScraperError)?;

    // Use the selector to find the matching element in the HTML
    let result = document.select(&selector).next();

    // Return the result or an error if no match was found
    result
        .map(|element| element.inner_html())
        .ok_or(ScraperError)
}

fn main() {
    // Example usage
    let url = "https://www.msl09.com.br/";
    let xpath = "h1"; // Replace this with your desired XPath expression

    match scrape_url(url, xpath) {
        Ok(result) => println!("Match found: {}", result),
        Err(_) => println!("No match found or error occurred"),
    }
}
