use dotenv::dotenv;
use serde_json::json;
use std::env;
use web_programming::{
    check_if_api_exists, create_gist, delete_gist, download_file_to_temp_directory,
    make_get_request, make_get_request_async, make_partial_download, post_file_to_paste_rs,
    query_github_api, ReverseDependencies,
};

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
    #[ignore]
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

        let response_status = delete_gist(&gist.id, &gh_user, &gh_pass).await;
        println!("Gist {} deleted! Status code: {}", gist.id, response_status);
        assert_eq!(response_status, 204);
    }

    #[test]
    fn test_reverse_dependencies() {
        for dep in ReverseDependencies::of("ring").unwrap() {
            println!("reverse dependency: {}", dep.unwrap().crate_id);
        }
        assert!(ReverseDependencies::of("ring").is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_download_file_to_temp_directory() {
        let dir_name = "test_download";
        let file_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
        let (dest, bytes) = download_file_to_temp_directory(dir_name, file_url)
            .await
            .unwrap();
        println!("Downloaded file: {:?}", dest);
        assert!(dest.file_type().is_file());
        println!("Downloaded {} bytes", bytes);
        assert!(bytes > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_post_file_to_paste_rs() {
        let message = "message.txt";
        let response_text = post_file_to_paste_rs(message).await.unwrap();
        println!("response_text: {}", response_text);
        assert!(response_text.contains("https://paste.rs"));
    }

    #[test]
    #[ignore]
    fn test_make_partial_download() {
        let chunk_size = 10240;
        let duration = 2;
        let output_file = make_partial_download(chunk_size, duration).unwrap();
        println!("output_file: {:?}", output_file);
        assert!(output_file.metadata().unwrap().is_file());
        assert_eq!(output_file.metadata().unwrap().len(), chunk_size as u64);
    }
}
