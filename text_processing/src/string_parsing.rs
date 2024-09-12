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
