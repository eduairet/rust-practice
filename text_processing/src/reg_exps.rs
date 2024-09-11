use lazy_static::lazy_static;
use regex::{Regex, RegexSetBuilder};
use shared::PhoneNumber;

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
    static ref USA_PHONE_NUMBER_REGEX: Regex = Regex::new(
        r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
    )
    .unwrap();
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

/// Extracts phone numbers from a text.
///
/// # Arguments
///
/// * `text` - A text containing phone numbers.
///
/// # Returns
///
/// A vector of phone numbers.
///
/// # Examples
///
/// ```
/// use text_processing::extract_phone_numbers;
///
/// let text = "Some numbers:
/// +1 (505) 881-9292,
/// 1.800.233.2010,";
///
/// let phone_numbers = extract_phone_numbers(text)
///    .into_iter()
///    .map(|m| m.to_string())
///    .collect::<Vec<_>>();
///
/// assert_eq!(
///    phone_numbers,
///    vec!["(505) 881-9292", "(800) 233-2010"]
/// );
/// ```
pub fn extract_phone_numbers(text: &str) -> Vec<PhoneNumber> {
    USA_PHONE_NUMBER_REGEX
        .captures_iter(text)
        .filter_map(|cap| {
            let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
            match groups {
                (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                    area: area.as_str(),
                    exchange: ext.as_str(),
                    subscriber: sub.as_str(),
                }),
                _ => None,
            }
        })
        .collect()
}

/// Matches several regexps in a text.
///
/// # Arguments
///
/// * `regexps` - A vector of regexps.
///
/// * `text` - A text to search.
///
/// # Returns
///
/// A vector of matched strings.
///
/// # Examples
///
/// ```
/// use text_processing::match_several_regexps;
///
/// let regexps = vec![r#"<[^>]*>"#, r#"</[^>]*>"#];
/// let text = "Regular paragraph\n<div>text</div><p>text</p><span>text</span>";
/// let matches = match_several_regexps(regexps, text);
/// assert_eq!(matches, vec!["<div>text</div><p>text</p><span>text</span>"]);
/// ```
pub fn match_several_regexps<'a>(regexps: Vec<&'a str>, text: &'a str) -> Vec<&'a str> {
    let set = RegexSetBuilder::new(&regexps)
        .case_insensitive(true)
        .build()
        .unwrap();

    text.lines().filter(|line| set.is_match(line)).collect()
}
