use reqwest::{blocking, header, Client, ClientBuilder, Result, StatusCode};
use serde_json::Value;
use shared::{ApiResponse, Dependency, Gist, User};
use std::time::Duration;

/// Make a GET request to the GitHub API
///
/// # Arguments
///
/// * `owner` - The owner of the repository
/// * `repo` - The repository name
///
/// # Returns
///
/// A tuple containing the status code, headers, and a vector of users
///
/// # Example
///
/// ```rust
/// use web_programming::query_github_api;
///
/// #[tokio::main]
/// async fn main() {
///    let owner = "rust-lang-nursery";
///    let repo = "rust-cookbook";
///    let (status, headers, body) = query_github_api(owner, repo).await;
///    assert_eq!(status, 200);
///    assert!(headers.contains_key("date"));
///    assert!(body.len() > 0);
/// }
pub async fn query_github_api(
    owner: &str,
    repo: &str,
) -> (StatusCode, header::HeaderMap, Vec<User>) {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers");
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .send()
        .await
        .unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let body: Vec<User> = res.json().await.unwrap();
    (status, headers, body)
}

/// Check if an API exists
///
/// # Arguments
///
/// * `request_url` - The URL to check
///
/// # Returns
///
/// A boolean indicating if the API exists
///
/// # Example
///
/// ```rust
/// use web_programming::check_if_api_exists;
///
/// #[tokio::main]
/// async fn main() {
///    let owner = "fonttools";
///    let repo = "fontbakery";
///    let request_url = format!("https://api.github.com/repos/{owner}/{repo}");
///    let exists = check_if_api_exists(&request_url).await;
///    assert!(exists);
/// }
pub async fn check_if_api_exists(request_url: &str) -> bool {
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();
    let response = client
        .head(request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .send()
        .await
        .unwrap();
    response.status().is_success()
}

/// Create Gist using GitHub API
///
/// # Arguments
///
/// * `gh_user` - GitHub username
/// * `gh_pass` - GitHub API token
/// * `gist_body` - JSON body of the Gist
///
/// # Returns
///
/// A Gist struct
///
/// # Example
///
/// ```ignore
/// use web_programming::create_gist;
///
/// #[tokio::main]
/// async fn main() {
///    let gh_user = env::var("GH_USER").unwrap();
///    let gh_pass = env::var("GH_PASS").unwrap();
///
///    let gist_body = json!({
///    "description": "Hello World Example",
///    "public": true,
///    "files": {
///         "main.py": {
///         "content": r#"print("hello world!")"#
///        }
///    }});
///
///    let gist = create_gist(&gh_user, &gh_pass, gist_body).await;
///    assert_eq!(
///        gist.html_url,
///        format!("https://gist.github.com/{}/{}", gh_user, gist.id)
///    );
/// }
/// ```
pub async fn create_gist(gh_user: &str, gh_pass: &str, gist_body: Value) -> Gist {
    let request_url = "https://api.github.com/gists";
    let response = Client::new()
        .post(request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .basic_auth(gh_user, Some(gh_pass))
        .json(&gist_body)
        .send()
        .await
        .unwrap();

    let gist: Gist = response.json().await.unwrap();

    gist
}

/// Delete Gist using GitHub API
///
/// # Arguments
///
/// * `gist_id` - The ID of the Gist
/// * `gh_user` - GitHub username
/// * `gh_pass` - GitHub API token
///
/// # Returns
///
/// A status code
///
/// # Example
///
/// ```ignore
/// use web_programming::delete_gist;
///
/// #[tokio::main]
/// async fn main() {
///    let gh_user = env::var("GH_USER").unwrap();
///    let gh_pass = env::var("GH_PASS").unwrap();
///
///    let response_status = delete_gist("gist_id", &gh_user, &gh_pass).await;
///    assert_eq!(response_status, 204);
/// }
/// ```
pub async fn delete_gist(gist_id: &str, gh_user: &str, gh_pass: &str) -> StatusCode {
    let request_url = format!("https://api.github.com/gists/{}", gist_id);
    let response = Client::new()
        .delete(&request_url)
        .header("User-Agent", "Rust Cookbook Client")
        .basic_auth(gh_user, Some(gh_pass))
        .send()
        .await
        .unwrap();

    response.status()
}

/// Get the reverse dependencies of a crate
///
/// # Example
///
/// ```rust
/// use web_programming::ReverseDependencies;
///
/// assert!(ReverseDependencies::of("ring").is_ok());
/// ```
pub struct ReverseDependencies {
    pub crate_id: String,
    pub dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    pub client: blocking::Client,
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
}

impl ReverseDependencies {
    pub fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
            crate_id: crate_id.to_owned(),
            dependencies: vec![].into_iter(),
            client: blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    pub fn try_next(&mut self) -> Result<Option<Dependency>> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
            self.crate_id, self.page, self.per_page
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "Rust Cookbook Client")
            .send()?
            .json::<ApiResponse>()?;
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
