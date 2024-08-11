use std::{error::Error, fs::File, io::Read};

/// Read the uptime from a file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<usize, Box<dyn Error>>` - A Result that holds the uptime in seconds or an error
///
/// # Example
///
/// ```
/// use error_handling::read_uptime;
///
/// let uptime = read_uptime("Cargo.toml");
/// assert_eq!(uptime.unwrap(), 69);
/// ```
pub fn read_uptime(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let mut uptime = String::new();

    let file = File::open(file_path)?.read_to_string(&mut uptime);

    let file_content = match file {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(file_content)
}
