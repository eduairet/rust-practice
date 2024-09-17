use url::{ParseError, Url};

/// Parses a URL from a string.
///
/// # Arguments
///
/// * `url` - The URL to parse.
///
/// # Example
///
/// ```
/// use web_programming::parse_url_from_string;
///
/// let url = "https://www.rust-lang.org/";
/// let result = parse_url_from_string(url);
/// assert!(result.is_ok());
/// ```
pub fn parse_url_from_string(url: &str) -> Result<Url, ParseError> {
    Url::parse(url)
}
