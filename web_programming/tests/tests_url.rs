use web_programming::{
    create_urls_from_base_url, extract_url_origin, get_base_url, parse_url_from_string,
};

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

    #[test]
    fn test_get_base_url() {
        let full_url = "https://github.com/rust-lang/cargo?asdf";
        let base = get_base_url(full_url).unwrap();
        assert_eq!(base.as_str(), "https://github.com/");
        println!("The base of the URL is: {}", base);
    }

    #[test]
    fn test_create_urls_from_base_url() {
        let base_url = "https://www.rust-lang.org/";
        let path = "learn";
        let result = create_urls_from_base_url(base_url, path);
        println!("{:?}", result);
        assert_eq!(result.unwrap().as_str(), "https://www.rust-lang.org/learn");
    }

    #[test]
    fn test_extract_url_origin() {
        let url = "https://www.rust-lang.org/learn";
        let result = extract_url_origin(url);
        match result {
            Ok((scheme, host, port)) => {
                println!("{:?} {:?} {:?}", scheme, host, port);
                assert_eq!(scheme, "https");
                assert_eq!(host.to_string(), "www.rust-lang.org");
                assert_eq!(port, 443);
            }
            Err(e) => panic!("Error extracting URL origin: {:?}", e),
        }
    }
}
