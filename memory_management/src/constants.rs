use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("eduairet", vec!["user", "admin", "contributor"]);
        map.insert("plandgrave", vec!["user", "contributor"]);
        map
    };
}

/// Shows the access of a given user from the `PRIVILEGES` constant
///
/// # Arguments
///
/// * `username` - A string slice that holds the username
///
/// # Returns
///
/// A tuple with the username and the access
///
/// # Examples
///
/// ```
/// use memory_management::show_access;
///
/// let username = "eduairet";
///
/// let (username, access) = show_access(username);
///
/// assert_eq!(username, Some("eduairet"));
/// assert_eq!(access, Some(&vec!["user", "admin", "contributor"]));
/// ```
pub fn show_access(username: &str) -> (Option<&str>, Option<&Vec<&str>>) {
    let access = PRIVILEGES.get(username);
    (Some(username), access)
}
