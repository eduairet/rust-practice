use text_processing::{
    extract_hashtags, extract_login_from_email, extract_phone_numbers, match_several_regexps,
    replace_patterns,
};

#[cfg(test)]
mod tests_reg_exps {

    use super::*;

    #[test]
    fn test_extract_login_from_email() {
        let email = "tester123@test.xyz";
        let login = extract_login_from_email(email).unwrap();
        println!("login: {}", &login);
        assert_eq!(login, "tester123");
    }

    #[test]
    fn test_extract_hashtags() {
        let text = "#hashtag1 #hashtag2 #hashtag3";
        let hashtags = extract_hashtags(text);
        println!("hashtags: {:?}", &hashtags);
        assert_eq!(hashtags, vec!["#hashtag1", "#hashtag2", "#hashtag3"]);
    }

    #[test]
    fn test_extract_phone_numbers() {
        let text = "Some numbers:
        +1 (505) 881-9292,
        +1 (505) 778-2212,
        (505) 881-9297,
        1 (202) 991-9534,
        5553920011,
        1.800.233.2010,";

        let phone_numbers = extract_phone_numbers(text)
            .into_iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>();

        println!("phone_numbers: {:?}", &phone_numbers);
        assert_eq!(
            phone_numbers,
            vec![
                "(505) 881-9292",
                "(505) 778-2212",
                "(505) 881-9297",
                "(202) 991-9534",
                "(555) 392-0011",
                "(800) 233-2010"
            ]
        );
    }

    #[test]
    fn test_match_several_regexps() {
        let regexps = vec![r#"<[^>]*>"#, r#"</[^>]*>"#];
        let text = "Regular paragraph\n<div>text</div><p>text</p><span>text</span>\nMixed paragraph <div>text</div>";
        let matches = match_several_regexps(regexps, text);
        println!("matches: {:?}", &matches);
        assert_eq!(
            matches,
            vec![
                "<div>text</div><p>text</p><span>text</span>",
                "Mixed paragraph <div>text</div>"
            ]
        );
    }

    #[test]
    fn test_replace_patterns() {
        let pattern_before = r#"<div[^>]*>([^<]*)</div>"#;
        let pattern_after = r#"<section>$1</section>"#;
        let text = "<div>text</div>";
        let replaced_text = replace_patterns(pattern_before, pattern_after, text);
        println!("replaced_text: {}", &replaced_text);
        assert_eq!(replaced_text, "<section>text</section>");
    }
}
