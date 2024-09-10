use text_processing::extract_login_from_email;

#[cfg(test)]
mod tests_regular_expressions {
    use super::*;

    #[test]
    fn test_extract_login_from_email() {
        let email = "tester123@test.xyz";
        let login = extract_login_from_email(email).unwrap();
        println!("login: {}", &login);
        assert_eq!(login, "tester123");
    }
}
