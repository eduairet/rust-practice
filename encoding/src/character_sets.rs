use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use std::str::Utf8Error;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Percent-encode a string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input.
///
/// # Returns
///
/// A string that holds the percent-encoded output.
///
/// # Example
///
/// ```
/// use encoding::percent_encode_string;
///
/// let decoded = "{ \"message\": \"Hello, world!\" }";
/// let expected = "{%20%22message%22:%20%22Hello,%20world!%22%20}";
///
/// let encoded = percent_encode_string(decoded);
/// assert_eq!(encoded, expected);
/// ```
pub fn percent_encode_string(input: &str) -> String {
    let iter = utf8_percent_encode(input, FRAGMENT);
    iter.collect()
}

/// Percent-decode a string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input.
///
/// # Returns
///
/// A string that holds the percent-decoded output.
///
/// # Example
///
/// ```
/// use encoding::percent_decode_string;
///
/// let encoded = "{%20%22message%22:%20%22Hello,%20world!%22%20}";
/// let expected = "{ \"message\": \"Hello, world!\" }";
///
/// let decoded = percent_decode_string(encoded).unwrap();
/// assert_eq!(decoded, expected);
/// ```
pub fn percent_decode_string(input: &str) -> Result<String, Utf8Error> {
    let iter = percent_decode(input.as_bytes());
    iter.decode_utf8().map(|cow| cow.to_string())
}