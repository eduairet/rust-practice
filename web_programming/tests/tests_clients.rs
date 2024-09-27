use serde_json::json;
use std::env;
use web_programming::{
    check_if_api_exists, create_gist, make_get_request, make_get_request_async, query_github_api,
};
use dotenv::dotenv;

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

    #[tokio::test]
    async fn test_query_github_api() {
        let owner = "rust-lang-nursery";
        let repo = "rust-cookbook";
        let (status, headers, body) = query_github_api(owner, repo).await;
        println!("status: {}", status);
        println!("headers: {:?}", headers);
        println!("body: {:?}", body);
        assert_eq!(status, 200);
        assert!(headers.contains_key("date"));
        assert!(body.len() > 0);
    }

    #[tokio::test]
    async fn test_check_if_api_exists() {
        let owner = "fonttools";
        let repo = "fontbakery";
        let request_url = format!("https://api.github.com/repos/{owner}/{repo}");
        let exists = check_if_api_exists(&request_url).await;
        print!("API exists: {}", exists);
        assert!(exists);
    }

    #[tokio::test]
    async fn test_create_delete_gist() {
        dotenv().ok();
        let gh_user = env::var("GH_USER").unwrap();
        let gh_pass = env::var("GH_PASS").unwrap();
        println!("gh_user: {}", gh_user);
        println!("gh_pass: {}", gh_pass);

        let gist_body = json!({
        "description": "Hello World Example",
        "public": true,
        "files": {
             "main.py": {
             "content": r#"print("hello world!")"#
            }
        }});

        let gist = create_gist(&gh_user, &gh_pass, gist_body).await;
        println!("gist: {:?}", gist.html_url);
        assert_eq!(
            gist.html_url,
            format!("https://gist.github.com/{}/{}", gh_user, gist.id) 
        );
    }
}
