use glob::{glob, glob_with, MatchOptions};
use same_file::is_same_file;
use std::{
    collections::HashMap,
    env::current_dir,
    error::Error,
    fs::{metadata, read_dir},
    io::Result as IoResult,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

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
/// ```ignore
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

/// Recursively finds duplicate file names in a given path
///
/// # Arguments
///
/// * `path` - A path to search for duplicate file names
///
/// # Returns
///
/// A boolean value that indicates if duplicate file names were found
///
/// # Examples
///
/// ```
/// use file_system::recursively_find_duplicate_file_names;
///
/// let path = ".";
///
/// let duplicates = recursively_find_duplicate_file_names(path).unwrap();
///
/// println!("{:?}", duplicates);
///
/// assert!(!duplicates);
/// ```
pub fn recursively_find_duplicate_file_names(path: &str) -> Result<bool, Box<dyn Error>> {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Recursively finds all files with a given predicate in a given path
///
/// # Arguments
///
/// * `path` - A path to search for files
/// * `predicate` - A predicate to filter files
///
/// # Returns
///
/// A vector of strings that holds the files that satisfy the predicate
///
/// # Examples
///
/// ```
/// use file_system::recursively_find_all_files_with_predicate;
///
/// let path = ".";
/// let predicate = ".rs";
///
/// let files = recursively_find_all_files_with_predicate(path, predicate).unwrap();
///
/// assert!(!files.is_empty());
/// ```
pub fn recursively_find_all_files_with_predicate(
    path: &str,
    predicate: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut filenames = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata().unwrap().modified().unwrap();

        if f_name.ends_with(predicate) && sec.elapsed().unwrap().as_secs() < 86400 {
            filenames.push(format!("{}", f_name));
        }
    }

    Ok(filenames)
}

/// Traverses directories skipping dotfiles
///
/// # Arguments
///
/// * `path` - A path to traverse
///
/// # Returns
///
/// A vector of DirEntry that holds the files
///
/// # Examples
///
/// ```
/// use file_system::traverse_directories_skipping_dotfiles;
///
/// let path = ".";
/// let files = traverse_directories_skipping_dotfiles(path).unwrap();
///
/// assert!(!files
///    .iter()
///    .any(|file| file.file_name().to_str().unwrap() == ".env"));
/// ```
pub fn traverse_directories_skipping_dotfiles(
    path: &str,
) -> Result<Vec<walkdir::DirEntry>, Box<dyn Error>> {
    pub fn is_not_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| entry.depth() == 0 || !s.starts_with("."))
            .unwrap_or(false)
    }
    let walker = WalkDir::new(path).into_iter();
    let files: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|e| is_not_hidden(e))
        .collect();
    Ok(files)
}

/// Recursively calculates file sizes at a given depth
///
/// # Arguments
///
/// * `path` - A path to calculate file sizes
/// * `min_depth` - An unsigned 64-bit integer that holds the minimum depth
/// * `max_depth` - An unsigned 64-bit integer that holds the maximum depth
///
/// # Returns
///
/// A string that holds the total size of the files
///
/// # Examples
///
/// ```
/// use file_system::recursively_calculate_file_sizes_at_given_depth;
///
/// let path = ".";
/// let min_depth = 1;
/// let max_depth = 2;
///
/// let file_sizes = recursively_calculate_file_sizes_at_given_depth(path, min_depth, max_depth);
///
/// assert!(file_sizes.len() > 0);
/// ```
pub fn recursively_calculate_file_sizes_at_given_depth(
    path: &str,
    min_depth: usize,
    max_depth: usize,
) -> String {
    let total_size = WalkDir::new(path)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    format!("Total size: {} bytes.", total_size)
}

/// Recursively finds all files in a given path
///
/// # Arguments
///
/// * `pattern` - A pattern to search for files
///
/// # Returns
///
/// A vector of strings that holds the files
///
/// # Examples
///
/// ```
/// use file_system::find_all_files_recursively;
///
/// let pattern = "**/*.rs";
/// let files = find_all_files_recursively(pattern).unwrap();
///
/// assert!(files.contains(&"tests/tests_directory_traversal.rs".to_string()));
/// ```
pub fn find_all_files_recursively(pattern: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();

    for entry in glob(pattern).unwrap() {
        match entry {
            Ok(path) => {
                files.push(path.display().to_string());
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(files)
}

/// Recursively finds all files in a given path ignoring case
///
/// # Arguments
///
/// * `pattern` - A pattern to search for files
///
/// # Returns
///
/// A vector of strings that holds the files
///
/// # Examples
///
/// ```
/// use file_system::find_all_files_recursively_ignoring_case;
///
/// let pattern = "**/*.RS";
/// let files = find_all_files_recursively_ignoring_case(pattern).unwrap();
///
/// assert!(files.contains(&"tests/tests_directory_traversal.rs".to_string()));
/// ```
pub fn find_all_files_recursively_ignoring_case(
    pattern: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let mut files = Vec::new();

    for entry in glob_with(pattern, options).unwrap() {
        match entry {
            Ok(path) => {
                files.push(path.display().to_string());
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(files)
}
