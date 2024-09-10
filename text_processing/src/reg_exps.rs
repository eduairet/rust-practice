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
}

/// Extracts login from an email address.
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
