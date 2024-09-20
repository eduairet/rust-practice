use web_programming::get_mime_type_from_string;

#[cfg(test)]
mod tests_media_types {
    use super::*;

    #[test]
    fn test_get_mime_type_from_string_valid() {
        let mime_type = get_mime_type_from_string("text/html");
        assert_eq!(mime_type.to_string(), "text/html".to_string());
    }

    #[test]
    fn test_get_mime_type_from_string_invalid() {
        let mime_type = get_mime_type_from_string("i n v a l i d");
        assert_eq!(
            mime_type.to_string(),
            "application/octet-stream".to_string()
        );
    }
}
