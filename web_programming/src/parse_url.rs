use std::io::{Error, ErrorKind};
use url::{Host, Origin, ParseError, Position::AfterPath, Url};

/// Parses a URL from a string.
///
/// # Arguments
///
/// * `string_url` - The URL to parse.
///
/// # Example
///
/// ```
/// use web_programming::parse_url_from_string;
///
/// let string_url = "https://www.rust-lang.org/";
/// let result = parse_url_from_string(string_url);
/// assert!(result.is_ok());
/// ```
pub fn parse_url_from_string(string_url: &str) -> Result<Url, ParseError> {
    Url::parse(string_url)
}

/// Gets the base URL of a URL.
///
/// # Arguments
///
/// * `full_url` - The URL string to get the base URL from.
///
/// # Example
///
/// ```
/// use web_programming::get_base_url;
///
/// let full_url = "https://github.com/rust-lang/cargo?asdf";
/// let base = get_base_url(full_url).unwrap();
/// assert_eq!(base.as_str(), "https://github.com/");
/// ```
pub fn get_base_url(full_url: &str) -> Result<Url, Error> {
    let mut url = Url::parse(full_url).unwrap();
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
pub fn create_urls_from_base_url(base_url: &str, path: &str) -> Result<Url, ParseError> {
    let base = Url::parse(base_url).expect("hardcoded URL is known to be valid");
    base.join(path)
}

/// Extracts the URL origin from a URL.
///
/// # Arguments
///
/// * `string_url` - The URL to extract the origin from.
///
/// # Example
///
/// ```
/// use web_programming::extract_url_origin;
/// use url::Host;
///
/// let string_url = "https://www.rust-lang.org/learn";
/// let result = extract_url_origin(string_url);
/// assert!(result.is_ok());
/// ```
pub fn extract_url_origin(string_url: &str) -> Result<(String, Host<String>, u16), Error> {
    let url = Url::parse(string_url).unwrap();
    let scheme = url.scheme().to_string();
    let host = url.host().unwrap().to_string();
    let port = url.port_or_known_default().unwrap();
    Ok((scheme, Host::parse(&host).unwrap(), port))
}

/// Extracts the URL origin from a URL.
///
///
/// # Arguments
///
/// * `string_url` - The URL to extract the origin from.
///
/// # Example
///
/// ```
/// use web_programming::extract_url_origin_alt;
/// use url::Origin;
///
/// let string_url = "ftp://rust-lang.org/examples";
/// let result = extract_url_origin_alt(string_url);
/// assert!(result.is_ok());
/// ```
pub fn extract_url_origin_alt(string_url: &str) -> Result<Origin, ParseError> {
    let url = Url::parse(string_url).unwrap();
    let result = url.origin();
    Ok(result)
}

/// Removes fragment identifiers and query pairs from a URL
///
/// /// # Arguments
///
/// * `string_url` - The URL to extract the origin from.
///
/// # Example
///
/// ```
/// use web_programming::remove_fragment_identifiers_and_query_pairs;
///
/// let string_url = "https://www.rust-lang.org/learn?query=string#frag";
/// let result = remove_fragment_identifiers_and_query_pairs(string_url).unwrap();
/// assert_eq!(result, "https://www.rust-lang.org/learn");
/// ```
pub fn remove_fragment_identifiers_and_query_pairs(string_url: &str) -> Result<String, ParseError> {
    let parsed = Url::parse(string_url).unwrap();
    let cleaned = parsed[..AfterPath].to_string();
    Ok(cleaned)
}
