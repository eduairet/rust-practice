use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

/// Reads a file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// A vector of strings that holds the lines of the file
///
/// # Examples
///
/// ```
/// use file_system::read_file_lines;
///
/// let file_path = "tests/test_read.txt";
/// let lines = read_file_lines(file_path);
/// assert_eq!(lines, vec!["Reading from test_read.txt", ":)"]);
/// ```
pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let file_buffered = BufReader::new(file);
    file_buffered.lines().map(|line| line.unwrap()).collect()
}

/// Writes to a file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
/// * `content` - A string slice that holds the content to write to the file
///
/// # Returns
///
/// A vector of strings that holds the lines of the file
///
/// # Examples
///
/// ```ignore
/// use file_system::write_to_file;
/// use std::{fs::remove_file, path::Path};
///
/// let file_path = "tests/test_write.txt";
/// let content = "Writing to test_write.txt";
/// let lines = write_to_file(file_path, content);
/// assert_eq!(lines, vec!["Writing to test_write.txt"]);
///
/// remove_file(file_path).unwrap();
/// assert!(!Path::new(file_path).exists());
/// ```
pub fn write_to_file(file_path: &str, content: &str) -> Vec<String> {
    let mut file = File::create(file_path).unwrap();
    write!(file, "{}", content).unwrap();
    read_file_lines(file_path)
}
