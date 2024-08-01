use encoding::{percent_decode_string, percent_encode_string};

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
}
