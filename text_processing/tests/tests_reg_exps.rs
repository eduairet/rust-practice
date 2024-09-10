use text_processing::{extract_hashtags, extract_login_from_email};

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
}
