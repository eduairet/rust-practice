use data_encoding::HEXUPPER;
use encoding::{percent_decode_string, percent_encode_string};
use url::form_urlencoded::{byte_serialize, parse};

#[cfg(test)]
mod tests_character_sets {
    use super::*;

    #[test]
    fn test_percent_encode_decode_string() {
        let input = "{ \"message\": \"Hello, world!\" }";
        let expected = "{%20%22message%22:%20%22Hello,%20world!%22%20}";

        let encoded = percent_encode_string(input);
        print!("encoded: {}\n", encoded);
        assert_eq!(encoded, expected);

        let decoded = percent_decode_string(&encoded).unwrap();
        print!("decoded: {}\n", decoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_as_application_x_www_form_urlencoded() {
        let input = "message=Hello, world!";
        let expected = "message%3DHello%2C+world%21";

        let encoded: String = byte_serialize(input.as_bytes()).collect();
        print!("encoded: {}\n", encoded);
        assert_eq!(encoded, expected);

        let decoded: String = parse(encoded.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();
        print!("decoded: {:?}\n", decoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_decode_hex() {
        let original = b"Hamburgerfontsiv";
        let expected = "48616D627572676572666F6E74736976";

        let encoded = HEXUPPER.encode(original);
        print!("encoded: {}\n", encoded);
        assert_eq!(encoded, expected);

        let decoded = HEXUPPER.decode(encoded.as_bytes()).unwrap();
        print!("decoded: {:?}\n", decoded);
        assert_eq!(decoded, original);
    }
}
