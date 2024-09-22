use mime::{Mime, APPLICATION_OCTET_STREAM};
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

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

/// Parse the MIME type from an HTTP response.
///
/// # Arguments
///
/// * `endpoint` - A string slice that holds the URL of the endpoint.
///
/// # Example
///
/// ```
/// use web_programming::parse_mime_type_from_http_response;
///
/// #[tokio::main]
/// async fn main() {
///    let endpoint = "https://www.rust-lang.org/logos/rust-logo-32x32.png";
///    let media_type = parse_mime_type_from_http_response(endpoint).await;
///    assert_eq!(media_type, "a PNG image".to_string());
/// }
pub async fn parse_mime_type_from_http_response(endpoint: &str) -> String {
    let response = reqwest::get(endpoint).await.unwrap();
    let headers = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            return "The response does not contain a Content-Type header.".to_string();
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str().unwrap()).unwrap();
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };

            return media_type.to_string();
        }
    };
}
