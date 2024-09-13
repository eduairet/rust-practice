use reqwest::get;
use select::{document::Document, predicate::Name};

/// Extracts all links from a website.
///
/// # Arguments
///
/// * `endpoint` - The URL of the website.
///
/// # Example
///
/// ```
/// use web_programming::extract_links_from_website;
///
/// #[tokio::main]
/// async fn main() {
///    let links = extract_links_from_website("https://www.rust-lang.org/").await;
///    assert!(links.len() > 0);
/// }
/// ```
pub async fn extract_links_from_website(endpoint: &str) -> Vec<String> {
    let response = get(endpoint).await.unwrap().text().await.unwrap();

    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|link| link.to_string())
        .collect()
}
