use web_programming::parse_url_from_string;

#[cfg(test)]
mod tests_url {
    use super::*;

    #[test]
    fn test_parse_url_from_string() {
        let url = "https://www.rust-lang.org/";
        let result = parse_url_from_string(url);
        println!("{:?}", result);
        assert_eq!(result.unwrap().host_str().unwrap(), "www.rust-lang.org");
    }
}
