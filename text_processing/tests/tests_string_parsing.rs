use text_processing::collect_unicode_graphemes;

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
}
