use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$
        "
    )
    .unwrap();
    static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
}

/// Extracts login from an email address.
///
/// # Arguments
///
/// * `email` - An email address.
///
/// # Returns
///
/// An option containing the login if it was found, or None otherwise.
///
/// # Examples
///
/// ```
/// use text_processing::extract_login_from_email;
///
/// let email = "abcd1234@test.xyz";
/// let login = extract_login_from_email(email).unwrap();
/// assert_eq!(login, "abcd1234");
/// ```
pub fn extract_login_from_email(email: &str) -> Option<&str> {
    EMAIL_REGEX
        .captures(email)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

/// Extracts hashtags from a text.
///
/// # Arguments
///
/// * `text` - A text containing hashtags.
///
/// # Returns
///
/// A vector of hashtags.
///
/// # Examples
///
/// ```
/// use text_processing::extract_hashtags;
///
/// let text = "#hashtag1 #hashtag2 #hashtag3";
/// let hashtags = extract_hashtags(text);
/// assert_eq!(hashtags, vec!["#hashtag1", "#hashtag2", "#hashtag3"]);
/// ```
pub fn extract_hashtags(text: &str) -> Vec<&str> {
    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}
