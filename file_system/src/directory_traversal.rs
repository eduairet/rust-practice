use same_file::is_same_file;
use std::{
    env::current_dir,
    error::Error,
    fs::{metadata, read_dir},
    io::Result as IoResult,
    path::{Path, PathBuf},
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

/// Finds loops in a given path
/// 
/// # Arguments
/// 
/// * `path` - A path to search for loops
/// 
/// # Returns
/// 
/// A tuple of two paths that form a loop
/// 
/// # Examples
/// 
/// ```
/// use file_system::find_loops_for_given_path;
/// use std::path::PathBuf;
/// 
/// let path = "/tmp/foo/bar/baz/qux/bar/baz";
/// let loops = find_loops_for_given_path(path).unwrap();
/// println!("{:?}", loops);
/// assert_eq!(
///    loops,
///    Some((
///       PathBuf::from("/tmp/foo"),
///       PathBuf::from("/tmp/foo/bar/baz/qux")
///    ))
/// );
/// ```
pub fn find_loops_for_given_path<P: AsRef<Path>>(path: P) -> IoResult<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = find_loops_for_given_path(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}
