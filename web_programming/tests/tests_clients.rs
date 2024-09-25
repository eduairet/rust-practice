use web_programming::{make_get_request, make_get_request_async};

#[cfg(test)]
mod tests_clients {
    use super::*;

    #[test]
    fn test_make_get_request() {
        let endpoint = "https://www.rust-lang.org/";
        let (status, headers, body) = make_get_request(endpoint);
        println!("status: {}", status);
        println!("headers: {:?}", headers);
        println!("body: {}", body.chars().take(100).collect::<String>());
        assert_eq!(status, 200);
        assert!(headers.contains_key("date"));
        assert!(body.contains("Rust"));
    }

    #[tokio::test]
    async fn test_make_get_request_async() {
        let endpoint = "https://www.rust-lang.org/";
        let (status, headers, body) = make_get_request_async(endpoint).await;
        println!("status: {}", status);
        println!("headers: {:?}", headers);
        println!("body: {}", body.chars().take(100).collect::<String>());
        assert_eq!(status, 200);
        assert!(headers.contains_key("date"));
        assert!(body.contains("Rust"));
    }
}
