use reqwest::{get, StatusCode};
use select::{document::Document, predicate::Name};
use std::collections::HashSet;
use url::{Position, Url};

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

/// Gets the base URL of a website.
///
/// # Arguments
///
/// * `url` - The URL of the website.
/// * `doc` - The document of the website.
///
/// # Example
///
/// ```
/// use web_programming::get_base_url;
/// use url::Url;
/// use select::document::Document;
///
/// #[tokio::main]
/// async fn main() {
///    let url = Url::parse("https://www.rust-lang.org/").unwrap();
///    let response = reqwest::get(url.as_ref()).await.unwrap().text().await.unwrap();
///    let document = Document::from(response.as_str());
///    let base_url = get_base_url(&url, &document).unwrap();
///   
///    assert_eq!(base_url, Url::parse("https://www.rust-lang.org/").unwrap());
/// }
/// ```
pub fn get_base_url(url: &Url, doc: &Document) -> Result<Url, url::ParseError> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

/// Checks if a link is broken.
///
/// # Arguments
///
/// * `url` - The URL to check.
///
/// # Example
///
/// ```
/// use web_programming::check_link;
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() {
///    let url = Url::parse("https://www.rust-lang.org/").unwrap();
///    let valid_link = check_link(&url).await.unwrap();
///    assert!(valid_link);
/// }
/// ```
pub async fn check_link(url: &Url) -> Result<bool, reqwest::Error> {
    let res = reqwest::get(url.as_ref()).await;
    match res {
        Ok(response) => Ok(response.status() != StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error checking link: {}", err);
            Ok(false)
        }
    }
}

/// Finds all broken links on a website.
///
/// # Arguments
///
/// * `endpoint` - The URL of the website.
///
/// # Example
///
/// ```
/// use web_programming::find_broken_links;
///
/// #[tokio::main]
/// async fn main() {
///    let links = find_broken_links("https://www.rust-lang.org/").await;
///    assert!(links.len() == 0);
/// }
pub async fn find_broken_links(endpoint: &str) -> Vec<String> {
    let url = Url::parse(endpoint).unwrap();
    let response = get(url.as_ref()).await.unwrap().text().await.unwrap();
    let document = Document::from(response.as_str());
    let base_url = get_base_url(&url, &document).unwrap();
    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    let mut tasks = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            match check_link(&link).await {
                Ok(true) => format!("OK: {}", link),
                _ => format!("Broken: {}", link),
            }
        }));
    }

    let mut broken_links = Vec::new();

    for task in tasks {
        let link = task.await.unwrap();
        if link.contains("Broken") {
            broken_links.push(link);
        }
    }

    broken_links
}
