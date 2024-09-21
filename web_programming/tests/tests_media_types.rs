use web_programming::{get_mime_type_from_filename, get_mime_type_from_string};

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

    #[test]
    fn test_get_mime_type_from_filename() {
        let files = vec![
            ("index.html", "text/html"),
            ("style.css", "text/css"),
            ("script.js", "text/javascript"),
            ("data.json", "application/json"),
            ("data.xml", "text/xml"),
            ("image.png", "image/png"),
            ("image.jpg", "image/jpeg"),
            ("image.jpeg", "image/jpeg"),
            ("image.gif", "image/gif"),
            ("image.svg", "image/svg+xml"),
            ("data.txt", "text/plain"),
            ("document.pdf", "application/pdf"),
            ("unknown", "application/octet-stream"),
        ];

        for (filename, expected) in files {
            let mime_type = get_mime_type_from_filename(filename);
            println!("{} -> {}", filename, mime_type);
            assert_eq!(mime_type.to_string(), expected.to_string());
        }
    }
}
