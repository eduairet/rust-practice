use reqwest::{blocking, header::HeaderMap, StatusCode};
use std::io::Read;

/// Makes a GET request to the given endpoint and returns the status code, headers, and body.
///
/// # Arguments
///
/// * `endpoint` - A string slice that holds the URL of the endpoint.
///
/// # Returns
///
/// A tuple containing the status code, headers, and body of the response.
///
/// # Example
///
/// ```
/// use web_programming::make_get_request;
///
/// let endpoint = "https://www.rust-lang.org/";
/// let (status, headers, body) = make_get_request(endpoint);
///
/// assert_eq!(status, 200);
/// assert!(headers.contains_key("date"));
/// assert!(body.contains("Rust"));
/// ```
pub fn make_get_request(endpoint: &str) -> (StatusCode, HeaderMap, String) {
    let mut res = blocking::get(endpoint).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    (status, headers, body)
}

/// Makes a GET request to the given endpoint and returns the status code, headers, and body.
///
/// # Arguments
///
/// * `endpoint` - A string slice that holds the URL of the endpoint.
///
/// # Returns
///
/// A tuple containing the status code, headers, and body of the response.
///
/// # Example
///
/// ```
/// use web_programming::make_get_request_async;
///
/// #[tokio::main]
/// async fn main() {
///    let endpoint = "https://www.rust-lang.org/";
///    let (status, headers, body) = make_get_request_async(endpoint).await;
///    assert_eq!(status, 200);
///    assert!(headers.contains_key("date"));
///    assert!(body.contains("Rust"));
/// }
pub async fn make_get_request_async(endpoint: &str) -> (StatusCode, HeaderMap, String) {
    let res = reqwest::get(endpoint).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let body = res.text().await.unwrap();
    (status, headers, body)
}
