use url::Url;
use web_programming::{get_base_url, parse_url_from_string};

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
        let full = "https://github.com/rust-lang/cargo?asdf";
        let url = Url::parse(full).unwrap();
        let base = get_base_url(url).unwrap();

        assert_eq!(base.as_str(), "https://github.com/");
        println!("The base of the URL is: {}", base);
    }
}
