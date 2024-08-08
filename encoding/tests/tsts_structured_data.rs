use serde_json::{json, Value};

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

        let parsed_data: Value = serde_json::from_str(json_data).unwrap();

        print!("Parsed data: {:?}", parsed_data);
        assert_eq!(parsed_data, expected);
    }
}
