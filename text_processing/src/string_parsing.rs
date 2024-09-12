use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

/// Collects all the Unicode graphemes in a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be processed.
///
/// # Returns
///
/// A vector of string slices, each representing a Unicode grapheme.
///
/// # Example
///
/// ```
/// use text_processing::collect_unicode_graphemes;
///
/// let text = "Garçon\r\n";
/// let graphemes = collect_unicode_graphemes(text);
/// assert_eq!(graphemes[3], "ç");
/// ```
pub fn collect_unicode_graphemes(text: &str) -> Vec<&str> {
    UnicodeSegmentation::graphemes(text, true).collect::<Vec<&str>>()
}

/// A struct that holds a full name.
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use text_processing::FullName;
///
/// let full_name = FullName::from_str("John Doe").unwrap();
/// assert_eq!(full_name.first_name, "John");
/// assert_eq!(full_name.last_name, "Doe");
/// ```
#[derive(Debug, PartialEq)]
pub struct FullName {
    pub first_name: String,
    pub last_name: String,
}
impl FromStr for FullName {
    type Err = std::string::String;

    // Get the first and last names from a full name.
    fn from_str(full_name: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Full name must have two parts".to_string());
        }
        Ok(FullName {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
        })
    }
}
