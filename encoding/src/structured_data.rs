use serde::Deserialize;
use std::collections::HashMap;

/// A struct representing a TOML configuration file.
///
/// # Example
///
/// ```
/// use encoding::Config;
///
/// let toml_data: &str = r#"
///    [package]
///    name = "encoding"
///    version = "0.1.0"
///    authors = ["Jane Doe"]
///
///    [dependencies]
///    serde = "1.0"
/// "#;
///
/// let package_info: Config = toml::from_str(toml_data).unwrap();
///
/// assert_eq!(package_info.package.name, "encoding");
/// assert_eq!(package_info.dependencies["serde"], "1.0");
/// ```
#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    pub dependencies: HashMap<String, String>,
}

/// A struct representing a package in a TOML configuration file.
///
/// # Example
///
/// ```
/// use encoding::Package;
///
/// let package_info: Package = toml::from_str(r#"
///    name = "encoding"
///    version = "0.1.0"
///    authors = ["Jane Doe"]
/// "#).unwrap();
///
/// assert_eq!(package_info.name, "encoding");
/// assert_eq!(package_info.version, "0.1.0");
/// assert_eq!(package_info.authors[0], "Jane Doe");
/// ```
#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}
