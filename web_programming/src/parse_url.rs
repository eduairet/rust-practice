use std::io::{Error, ErrorKind};
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

/// Gets the base URL of a URL.
///
/// # Arguments
///
/// * `url` - The URL to get the base URL from.
///
/// # Example
///
/// ```
/// use url::Url;
/// use web_programming::get_base_url;
///
/// let full = "https://github.com/rust-lang/cargo?asdf";
/// let mut url = Url::parse(full).unwrap();
/// let base = get_base_url(url).unwrap();
///
/// assert_eq!(base.as_str(), "https://github.com/");
/// ```
pub fn get_base_url(mut url: Url) -> Result<Url, Error> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::InvalidInput, "Cannot be a base URL"));
        }
    }

    url.set_query(None);

    Ok(url)
}

/// Creates a URL from a base URL and a path.
///
/// # Arguments
///
/// * `base_url` - The base URL.
/// * `path` - The path to append to the base URL.
///
/// # Example
///
/// ```
/// use web_programming::create_urls_from_base_url;
/// use url::Url;
///
/// let base_url = "https://www.rust-lang.org/";
/// let path = "learn";
/// let result = create_urls_from_base_url(base_url, path);
/// assert!(result.is_ok());
/// ```
pub fn create_urls_from_base_url(base_url: &str, path: &str) -> Result<Url, url::ParseError> {
    let base = Url::parse(base_url).expect("hardcoded URL is known to be valid");
    base.join(path)
}
