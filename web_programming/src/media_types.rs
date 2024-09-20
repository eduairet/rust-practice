use mime::{Mime, APPLICATION_OCTET_STREAM};

/// Get a `Mime` type from a string.
///
/// # Arguments
///
/// * `string` - A string slice that holds the MIME type.
///
/// # Example
///
/// ```
/// use web_programming::get_mime_type_from_string;
///
/// let mime_type = get_mime_type_from_string("text/html");
/// assert_eq!(mime_type.to_string(), "text/html".to_string());
/// ```
pub fn get_mime_type_from_string(string: &str) -> Mime {
    string.parse::<Mime>().unwrap_or(APPLICATION_OCTET_STREAM)
}
