use serde_json::{json, Value as JsonValue};
use toml::Value as TomlValue;

#[cfg(test)]
mod tests_structured_data {

    use super::*;

    #[test]
    fn test_serialize_deserialize_unstructured_json() {
        let json_data = r#"{
            "name":"John",
            "age":30,
            "location": {
                "city": "New York",
                "state": "NY"
            },
            "hobbies": [
                "Reading",
                "Traveling"
            ]
        }"#;

        let expected = json!({
            "name":"John",
            "age":30,
            "location": {
                "city": "New York",
                "state": "NY"
            },
            "hobbies": [
                "Reading",
                "Traveling"
            ]
        });

        let parsed_data: JsonValue = serde_json::from_str(json_data).unwrap();

        print!("Parsed data: {:?}", parsed_data);
        assert_eq!(parsed_data, expected);
    }

    #[test]
    fn test_deserialize_toml() {
        let toml_data = r#"
            [package]
            name = "encoding"
            version = "0.1.0"
            authors = ["Eduardo Aire <eat@beef.pnk>"]

            [dependencies]
            serde = "1.0"
        "#;

        let package_info: TomlValue = toml::from_str(toml_data).unwrap();

        print!("Package info: {:?}", package_info);
        assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
        assert_eq!(package_info["package"]["name"].as_str(), Some("encoding"));
    }
}
