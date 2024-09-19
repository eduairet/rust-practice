use url::{Host, Origin};
use web_programming::{
    create_urls_from_base_url, extract_url_origin, extract_url_origin_alt, get_base_url,
    parse_url_from_string, remove_fragment_identifiers_and_query_pairs,
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

    #[test]
    fn test_extract_url_origin_alt() {
        let string_url = "ftp://rust-lang.org/examples";
        let expected_scheme = "ftp".to_owned();
        let expected_host = Host::Domain("rust-lang.org".to_owned());
        let expected_port = 21;
        let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

        let result = extract_url_origin_alt(string_url).unwrap();
        println!("{:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_fragment_identifiers_and_query_pairs() {
        let url = "https://www.rust-lang.org/learn?query=string#frag";
        let result = remove_fragment_identifiers_and_query_pairs(url).unwrap();
        println!("{:?}", result);
        assert_eq!(result, "https://www.rust-lang.org/learn");
    }
}
