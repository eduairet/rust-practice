use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Error;

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

/// A struct representing a payload with a kind and a value.
///
/// # Example
///
/// ```
/// use encoding::Payload;
///
/// let payload = Payload {
///    kind: 0,
///    value: 42,
/// };
///
/// assert_eq!(payload.kind, 0);
/// assert_eq!(payload.value, 42);
/// ```
#[derive(Default, PartialEq, Debug)]
pub struct Payload {
    pub kind: u8,
    pub value: u16,
}

/// Encodes a payload into a byte vector using little-endian encoding.
///
/// # Arguments
///
/// * `payload` - A reference to a `Payload` struct.
///
/// # Returns
///
/// A `Result` containing a `Vec<u8>` if the encoding was successful, or an `Error` if it failed.
///
/// # Example
///
/// ```
/// use encoding::{encode_little_endian, Payload};
///
/// let payload = Payload {
///    kind: 0,
///    value: 42,
/// };
///
/// let encoded_bytes = encode_little_endian(&payload).unwrap();
/// assert_eq!(encoded_bytes, vec![0, 42, 0]);
/// ```
pub fn encode_little_endian(payload: &Payload) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

/// Decodes a byte slice into a payload using little-endian encoding.
///
/// # Arguments
///
/// * `bytes` - A reference to a byte slice.
///
/// # Returns
///
/// A `Result` containing a `Payload` if the decoding was successful, or an `Error` if it failed.
///
/// # Example
///
/// ```
/// use encoding::{decode_little_endian, Payload};
///
/// let bytes = vec![0, 42, 0];
/// let payload = decode_little_endian(&bytes).unwrap();
///
/// assert_eq!(payload.kind, 0);
/// assert_eq!(payload.value, 42);
/// ```
pub fn decode_little_endian(mut bytes: &[u8]) -> Result<Payload, Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}
