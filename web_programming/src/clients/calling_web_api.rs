use reqwest::{header, ClientBuilder, StatusCode};
use shared::User;
use std::time::Duration;

/// Make a GET request to the GitHub API
///
/// # Arguments
///
/// * `owner` - The owner of the repository
/// * `repo` - The repository name
///
/// # Returns
///
/// A tuple containing the status code, headers, and a vector of users
///
/// # Example
///
/// ```rust
/// use web_programming::query_github_api;
///
/// #[tokio::main]
/// async fn main() {
///    let owner = "rust-lang-nursery";
///    let repo = "rust-cookbook";
///    let (status, headers, body) = query_github_api(owner, repo).await;
///    assert_eq!(status, 200);
///    assert!(headers.contains_key("date"));
///    assert!(body.len() > 0);
/// }
pub async fn query_github_api(
    owner: &str,
    repo: &str,
) -> (StatusCode, header::HeaderMap, Vec<User>) {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers");
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .send()
        .await
        .unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let body: Vec<User> = res.json().await.unwrap();
    (status, headers, body)
}

/// Check if an API exists
///
/// # Arguments
///
/// * `request_url` - The URL to check
///
/// # Returns
///
/// A boolean indicating if the API exists
///
/// # Example
///
/// ```rust
/// use web_programming::check_if_api_exists;
///
/// #[tokio::main]
/// async fn main() {
///    let owner = "fonttools";
///    let repo = "fontbakery";
///    let request_url = format!("https://api.github.com/repos/{owner}/{repo}");
///    let exists = check_if_api_exists(&request_url).await;
///    assert!(exists);
/// }
pub async fn check_if_api_exists(request_url: &str) -> bool {
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();
    let response = client
        .head(request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .send()
        .await
        .unwrap();
    response.status().is_success()
}
