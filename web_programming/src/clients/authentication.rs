use reqwest::blocking::{Client, Response};
use std::error::Error;

/// This function sends a basic authentication request to the httpbin.org website.
///
/// # Arguments
///
/// * `user_name` - A string slice that holds the user name.
/// * `password` - An Option<String> that holds the password.
///
/// # Returns
///
/// A Response that holds the response from the server.
///
/// # Example
///
/// ```
/// use web_programming::clients::authentication::basic_authentication;
///
/// let user_name = "user";
/// let password = Some("password".to_string());
/// let response = basic_authentication(user_name, password).unwrap();
/// assert!(response.status().is_success());
/// ```
pub fn basic_authentication(
    user_name: &str,
    password: Option<String>,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send()
        .unwrap();

    Ok(response)
}
