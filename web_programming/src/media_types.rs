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

/// Get a `Mime` type from a filename.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the filename.
///
/// # Example
///
/// ```
/// use web_programming::get_mime_type_from_filename;
///
/// let file = "index.html";
/// let mime_type = get_mime_type_from_filename(file);
/// assert_eq!(mime_type.to_string(), "text/html".to_string());
/// ```
pub fn get_mime_type_from_filename(filename: &str) -> Mime {
    let parts = filename.split('.').collect::<Vec<&str>>();
    let res = match parts.last() {
        Some(v) => match *v {
            "html" => mime::TEXT_HTML,
            "css" => mime::TEXT_CSS,
            "js" => mime::TEXT_JAVASCRIPT,
            "json" => mime::APPLICATION_JSON,
            "xml" => mime::TEXT_XML,
            "png" => mime::IMAGE_PNG,
            "jpg" | "jpeg" => mime::IMAGE_JPEG,
            "gif" => mime::IMAGE_GIF,
            "svg" => mime::IMAGE_SVG,
            "txt" => mime::TEXT_PLAIN,
            "pdf" => mime::APPLICATION_PDF,
            _ => mime::APPLICATION_OCTET_STREAM,
        },
        None => mime::TEXT_PLAIN,
    };
    res
}
