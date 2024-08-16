use std::{
    env::current_dir,
    error::Error,
    fs::{metadata, read_dir},
};

/// Searches for files that were modified within the last `hours_back` hours
///
/// # Arguments
///
/// * `hours_back` - An unsigned 64-bit integer that holds the number of hours to search back
///
/// # Returns
///
/// A vector of strings that holds the modified files
/// Returns an error if the current directory cannot be accessed
///
/// # Examples
///
/// ```
/// use file_system::search_modified_files_in_current_dir;
///
/// let hours_back = 24;
/// let modified_files = search_modified_files_in_current_dir(hours_back);
/// println!("{:?}", modified_files);
/// assert!(modified_files.is_ok());
/// ```
pub fn search_modified_files_in_current_dir(
    hours_back: u64,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut modified_files = Vec::new();

    let current_dir = current_dir().unwrap();

    for entry in read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = metadata(&path).unwrap();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

        if last_modified < hours_back * 3600 && metadata.is_file() {
            let file = format!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename").unwrap()
            );

            modified_files.push(String::from(file));
        }
    }

    Ok(modified_files)
}
