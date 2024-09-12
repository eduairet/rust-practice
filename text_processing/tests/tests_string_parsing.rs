use std::str::FromStr;
use text_processing::{collect_unicode_graphemes, FullName};

#[cfg(test)]
mod tests_string_parsing {

    use super::*;

    #[test]
    fn test_collect_unicode_graphemes() {
        let text = "a̐éö̲\r\n";
        let graphemes = collect_unicode_graphemes(text);
        println!("{:?}", graphemes);
        assert_eq!(graphemes, vec!["a̐", "é", "ö̲", "\r\n"]);
    }

    #[test]
    fn test_full_name_from_str() {
        let full_name = FullName::from_str("John Doe").unwrap();
        println!("{:?}", full_name);
        assert_eq!(full_name.first_name, "John");
        assert_eq!(full_name.last_name, "Doe");
    }
}
